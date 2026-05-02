//! This module defines the `MonoBindings` struct, which holds function pointers to the Mono API exports we use
//! and provides safe wrapper methods to call them. It also includes helper functions to resolve these exports
//! from the Mono DLL.

use std::ffi::{CString, c_char, c_void};
use std::{mem, ptr};

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
}

impl MonoBindings {
    /// Resolve all required Mono API exports from the given module handle.
    ///
    /// # Errors
    ///
    /// Returns `MonoError::FnNotFound` if any required export is missing.
    pub fn new(module: HMODULE) -> Result<Self> {
        let exports = Self {
            mono_get_root_domain: resolve(module, "mono_get_root_domain")?,
            mono_thread_attach: resolve(module, "mono_thread_attach")?,
            mono_thread_detach: resolve(module, "mono_thread_detach")?,
            mono_assembly_foreach: resolve(module, "mono_assembly_foreach")?,
            mono_assembly_get_image: resolve(module, "mono_assembly_get_image")?,
            mono_image_get_name: resolve(module, "mono_image_get_name")?,
            mono_class_from_name: resolve(module, "mono_class_from_name")?,
            mono_class_get_field_from_name: resolve(module, "mono_class_get_field_from_name")?,
            mono_field_get_offset: resolve(module, "mono_field_get_offset")?,
            mono_class_get_method_from_name: resolve(module, "mono_class_get_method_from_name")?,
            mono_runtime_invoke: resolve(module, "mono_runtime_invoke")?,
            mono_array_length: resolve(module, "mono_array_length")?,
            mono_array_addr_with_size: resolve(module, "mono_array_addr_with_size")?,
            mono_class_get_type: resolve(module, "mono_class_get_type")?,
            mono_type_get_object: resolve(module, "mono_type_get_object")?,
            mono_string_to_utf8: resolve(module, "mono_string_to_utf8")?,
            mono_free: resolve(module, "mono_free")?,
            mono_object_unbox: resolve(module, "mono_object_unbox")?,
            mono_class_vtable: resolve(module, "mono_class_vtable")?,
            mono_field_static_get_value: resolve(module, "mono_field_static_get_value")?,
        };

        Ok(exports)
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
    ) -> Result<*mut c_void> {
        let result = unsafe { (self.mono_runtime_invoke)(method, obj, params, exc) };

        if result.is_null() {
            // a null result can indicate either a void return or a thrown exception,
            // so we check the exc pointer to be sure
            let exc_ptr = unsafe { *exc };
            if !exc_ptr.is_null() {
                // the method threw a managed exception, which we treat as a non-error case and return None
                // TODO: provide a stronger API for inspecting the error
                return Ok(ptr::null_mut());
            }
        }

        Ok(result)
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
