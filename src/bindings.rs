//! This module defines the `MonoBindings` struct, which holds function pointers to the Mono API exports we use
//! and provides safe wrapper methods to call them. It also includes helper functions to resolve these exports
//! from the Mono DLL.

use std::ffi::{CString, c_char, c_void};
use std::mem;

use windows::{
    Win32::{Foundation::HMODULE, System::LibraryLoader::GetProcAddress},
    core::PCSTR,
};

use super::types::MonoFunc;
use super::{MonoError, Result};

type MonoInvokeFn = unsafe extern "C" fn(
    *mut c_void,
    *mut c_void,
    *mut *mut c_void,
    *mut *mut c_void,
) -> *mut c_void;

/// Resolves all fields of `MonoBindings` by name, using `stringify!` to match each field name to
/// its Mono export. This keeps `new()` compact regardless of how many exports are added.
macro_rules! resolve_bindings {
    ($module:ident => $($field:ident),+ $(,)?) => {
        Ok(Self {
            $( $field: resolve($module, stringify!($field))?, )+
        })
    };
}

/// Struct holding function pointers to the Mono API exports we use.
/// This allows us to resolve them once at startup and call them safely everywhere else without
/// repeated lookups. The field names mirror the export names for clarity, but the struct itself
/// is private and only accessed through safe wrapper methods.
#[allow(clippy::struct_field_names)]
pub(crate) struct MonoBindings {
    mono_get_root_domain: unsafe extern "C" fn() -> *mut c_void,
    mono_thread_attach: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    mono_thread_detach: unsafe extern "C" fn(*mut c_void),
    mono_assembly_foreach: unsafe extern "C" fn(MonoFunc, *mut c_void),
    mono_assembly_get_image: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    mono_image_get_name: unsafe extern "C" fn(*mut c_void) -> *const c_char,
    mono_class_from_name:
        unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char) -> *mut c_void,
    mono_class_get_field_from_name: unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void,
    mono_field_get_offset: unsafe extern "C" fn(*mut c_void) -> u32,
    mono_class_get_method_from_name:
        unsafe extern "C" fn(*mut c_void, *const c_char, i32) -> *mut c_void,
    mono_runtime_invoke: MonoInvokeFn,
    mono_array_length: unsafe extern "C" fn(*mut c_void) -> usize,
    mono_array_addr_with_size: unsafe extern "C" fn(*mut c_void, i32, usize) -> *mut c_void,
    mono_class_get_type: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    mono_type_get_object: unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void,
    mono_string_to_utf8: unsafe extern "C" fn(*mut c_void) -> *mut c_char,
    mono_free: unsafe extern "C" fn(*mut c_void),
    mono_object_unbox: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    mono_class_vtable: unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void,
    mono_field_static_get_value: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void),
    mono_object_new: unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void,
    mono_domain_assembly_open: unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void,
    mono_string_new: unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void,
    mono_class_get_fields: unsafe extern "C" fn(*mut c_void, *mut *mut c_void) -> *mut c_void,
    mono_class_get_methods: unsafe extern "C" fn(*mut c_void, *mut *mut c_void) -> *mut c_void,
    mono_type_get_type: unsafe extern "C" fn(*mut c_void) -> u32,
    mono_field_get_name: unsafe extern "C" fn(*mut c_void) -> *const c_char,
    mono_field_get_type: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    mono_method_get_name: unsafe extern "C" fn(*mut c_void) -> *const c_char,
    mono_image_open_from_data: unsafe extern "C" fn(
        data: *mut c_char,
        data_len: u32,
        need_copy: i32,
        status: *mut i32,
    ) -> *mut c_void,
    mono_assembly_load_from_full:
        unsafe extern "C" fn(*mut c_void, *const c_char, *mut i32, i32) -> *mut c_void,
    mono_assembly_close: unsafe extern "C" fn(*mut c_void),
    mono_image_strerror: unsafe extern "C" fn(i32) -> *const c_char,
    mono_object_get_class: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    mono_class_get_name: unsafe extern "C" fn(*mut c_void) -> *const c_char,
}

impl MonoBindings {
    /// Resolve all required Mono API exports from the given module handle.
    ///
    /// # Errors
    ///
    /// Returns `MonoError::FnNotFound` if any required export is missing.
    pub fn new(module: HMODULE) -> Result<Self> {
        resolve_bindings!(module =>
            mono_get_root_domain, mono_thread_attach, mono_thread_detach,
            mono_assembly_foreach, mono_assembly_get_image, mono_image_get_name,
            mono_class_from_name, mono_class_get_field_from_name, mono_field_get_offset,
            mono_class_get_method_from_name, mono_runtime_invoke, mono_array_length,
            mono_array_addr_with_size, mono_class_get_type, mono_type_get_object,
            mono_string_to_utf8, mono_free, mono_object_unbox, mono_class_vtable,
            mono_field_static_get_value, mono_object_new, mono_domain_assembly_open,
            mono_string_new, mono_class_get_fields, mono_class_get_methods,
            mono_type_get_type, mono_field_get_name, mono_field_get_type, mono_method_get_name,
            mono_image_open_from_data, mono_assembly_load_from_full, mono_assembly_close,
            mono_image_strerror, mono_object_get_class, mono_class_get_name
        )
    }

    pub fn get_root_domain(&self) -> *mut c_void {
        unsafe { (self.mono_get_root_domain)() }
    }

    pub fn thread_attach(&self, domain: *mut c_void) -> *mut c_void {
        unsafe { (self.mono_thread_attach)(domain) }
    }

    pub fn thread_detach(&self, thread: *mut c_void) {
        unsafe { (self.mono_thread_detach)(thread) }
    }

    pub fn assembly_foreach(&self, func: MonoFunc, user_data: *mut c_void) {
        unsafe { (self.mono_assembly_foreach)(func, user_data) }
    }

    pub fn assembly_get_image(&self, assembly: *mut c_void) -> *mut c_void {
        unsafe { (self.mono_assembly_get_image)(assembly) }
    }

    pub fn image_get_name(&self, image: *mut c_void) -> *const c_char {
        unsafe { (self.mono_image_get_name)(image) }
    }

    pub fn image_open_from_data(
        &self,
        data: *mut c_char,
        data_len: u32,
        need_copy: i32,
        status: *mut i32,
    ) -> *mut c_void {
        unsafe { (self.mono_image_open_from_data)(data, data_len, need_copy, status) }
    }

    pub fn class_from_name(
        &self,
        image: *mut c_void,
        namespace: *const c_char,
        name: *const c_char,
    ) -> *mut c_void {
        unsafe { (self.mono_class_from_name)(image, namespace, name) }
    }

    pub fn class_get_field_from_name(
        &self,
        class: *mut c_void,
        name: *const c_char,
    ) -> *mut c_void {
        unsafe { (self.mono_class_get_field_from_name)(class, name) }
    }

    pub fn field_get_offset(&self, field: *mut c_void) -> u32 {
        unsafe { (self.mono_field_get_offset)(field) }
    }

    pub fn class_get_method_from_name(
        &self,
        class: *mut c_void,
        name: *const c_char,
        param_count: i32,
    ) -> *mut c_void {
        unsafe { (self.mono_class_get_method_from_name)(class, name, param_count) }
    }

    /// Invokes a Mono method via the SEH shim.
    pub fn runtime_invoke(
        &self,
        method: *mut c_void,
        obj: *mut c_void,
        params: *mut *mut c_void,
        exc: *mut *mut c_void,
    ) -> *mut c_void {
        unsafe { (self.mono_runtime_invoke)(method, obj, params, exc) }
    }

    pub fn array_length(&self, array: *mut c_void) -> usize {
        unsafe { (self.mono_array_length)(array) }
    }

    pub fn array_addr_with_size(&self, array: *mut c_void, size: i32, index: usize) -> *mut c_void {
        unsafe { (self.mono_array_addr_with_size)(array, size, index) }
    }

    pub fn class_get_type(&self, class: *mut c_void) -> *mut c_void {
        unsafe { (self.mono_class_get_type)(class) }
    }

    pub fn type_get_object(&self, domain: *mut c_void, type_ptr: *mut c_void) -> *mut c_void {
        unsafe { (self.mono_type_get_object)(domain, type_ptr) }
    }

    pub fn string_to_utf8(&self, string: *mut c_void) -> *mut c_char {
        unsafe { (self.mono_string_to_utf8)(string) }
    }

    pub fn free(&self, ptr: *mut c_void) {
        unsafe { (self.mono_free)(ptr) }
    }

    pub fn object_unbox(&self, obj: *mut c_void) -> *mut c_void {
        unsafe { (self.mono_object_unbox)(obj) }
    }

    pub fn class_vtable(&self, domain: *mut c_void, class: *mut c_void) -> *mut c_void {
        unsafe { (self.mono_class_vtable)(domain, class) }
    }

    pub fn field_static_get_value(
        &self,
        vtable: *mut c_void,
        field: *mut c_void,
        value: *mut c_void,
    ) {
        unsafe { (self.mono_field_static_get_value)(vtable, field, value) }
    }

    pub fn object_new(&self, domain: *mut c_void, klass: *mut c_void) -> *mut c_void {
        unsafe { (self.mono_object_new)(domain, klass) }
    }

    pub fn domain_assembly_open(&self, domain: *mut c_void, name: *const c_char) -> *mut c_void {
        unsafe { (self.mono_domain_assembly_open)(domain, name) }
    }

    pub fn string_new(&self, domain: *mut c_void, text: *const c_char) -> *mut c_void {
        unsafe { (self.mono_string_new)(domain, text) }
    }

    /// Collects all fields of `klass` using Mono's iterator protocol.
    pub fn class_get_fields(&self, klass: *mut c_void) -> Vec<*mut c_void> {
        let mut iter: *mut c_void = std::ptr::null_mut();
        let mut out = Vec::new();
        loop {
            let ptr = unsafe { (self.mono_class_get_fields)(klass, std::ptr::addr_of_mut!(iter)) };
            if ptr.is_null() {
                break;
            }
            out.push(ptr);
        }
        out
    }

    /// Collects all methods of `klass` using Mono's iterator protocol.
    pub fn class_get_methods(&self, klass: *mut c_void) -> Vec<*mut c_void> {
        let mut iter: *mut c_void = std::ptr::null_mut();
        let mut out = Vec::new();
        loop {
            let ptr = unsafe { (self.mono_class_get_methods)(klass, std::ptr::addr_of_mut!(iter)) };
            if ptr.is_null() {
                break;
            }
            out.push(ptr);
        }
        out
    }

    pub fn type_get_type(&self, mono_type: *mut c_void) -> u32 {
        unsafe { (self.mono_type_get_type)(mono_type) }
    }

    pub fn field_get_name(&self, field: *mut c_void) -> *const c_char {
        unsafe { (self.mono_field_get_name)(field) }
    }

    pub fn field_get_type(&self, field: *mut c_void) -> *mut c_void {
        unsafe { (self.mono_field_get_type)(field) }
    }

    pub fn method_get_name(&self, method: *mut c_void) -> *const c_char {
        unsafe { (self.mono_method_get_name)(method) }
    }

    pub fn assembly_load_from_full(
        &self,
        image: *mut c_void,
        base_dir: *const c_char,
        status: *mut i32,
        refonly: i32,
    ) -> *mut c_void {
        unsafe { (self.mono_assembly_load_from_full)(image, base_dir, status, refonly) }
    }

    pub fn assembly_close(&self, assembly: *mut c_void) {
        unsafe { (self.mono_assembly_close)(assembly) }
    }

    pub fn image_strerror(&self, status: i32) -> *const c_char {
        unsafe { (self.mono_image_strerror)(status) }
    }

    pub fn object_get_class(&self, obj: *mut c_void) -> *mut c_void {
        unsafe { (self.mono_object_get_class)(obj) }
    }

    pub fn class_get_name(&self, klass: *mut c_void) -> *const c_char {
        unsafe { (self.mono_class_get_name)(klass) }
    }
}

unsafe impl Send for MonoBindings {}
unsafe impl Sync for MonoBindings {}

/// Try to resolve a function pointer from the module, returning `None` if not found.
#[must_use]
fn try_resolve<F: Copy>(module: HMODULE, name: &'static str) -> Option<F> {
    let c_name = CString::new(name).ok()?;
    let proc = unsafe { GetProcAddress(module, PCSTR(c_name.as_ptr().cast())) }?;
    Some(unsafe { mem::transmute_copy(&proc) })
}

/// Resolve a function pointer from the module, returning an error if not found.
///
/// # Errors
///
/// Returns `MonoError::FnNotFound` if the export is missing.
fn resolve<F: Copy>(module: HMODULE, name: &'static str) -> Result<F> {
    try_resolve(module, name).ok_or(MonoError::FnNotFound(name))
}
