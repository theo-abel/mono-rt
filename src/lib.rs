mod bindings;
mod error;
mod guard;
mod types;

use std::{ffi::c_void, sync::OnceLock};

use windows::{Win32::System::LibraryLoader::GetModuleHandleW, core::PCWSTR};

use bindings::MonoBindings;
pub use error::{MonoError, Result};
pub use guard::MonoThreadGuard;
pub use types::{
    MonoArray, MonoAssembly, MonoClass, MonoClassField, MonoDomain, MonoFunc, MonoImage,
    MonoMethod, MonoObject, MonoString, MonoThread, MonoType, MonoVTable, TypeKind, Value,
};

pub mod prelude {
    pub use crate::{
        MonoArray, MonoAssembly, MonoClass, MonoClassField, MonoDomain, MonoError, MonoImage,
        MonoMethod, MonoObject, MonoString, MonoThread, MonoThreadGuard, MonoType, MonoVTable,
        Result, TypeKind, Value,
    };
}

static BINDINGS: OnceLock<MonoBindings> = OnceLock::new();

/// Initialize the Mono API bindings by resolving exports from the given module.
/// Must be called before using any Mono types.
///
/// # Errors
///
/// Returns [`MonoError`] if the dll or required exports are missing.
pub fn init(module_name: &str) -> Result<()> {
    let wide: Vec<u16> = module_name.encode_utf16().chain(std::iter::once(0)).collect();
    let module = unsafe { GetModuleHandleW(PCWSTR::from_raw(wide.as_ptr())) }
        .map_err(|_| MonoError::DllNotFound(module_name.to_owned()))?;

    let exports = MonoBindings::new(module)?;

    BINDINGS
        .set(exports)
        .map_err(|_| MonoError::AlreadyInitialized)?;
    Ok(())
}

/// Get a reference to the resolved Mono API exports.
///
/// # Panics
///
/// Panics if `init()` has not been called yet.
pub(crate) fn api() -> Result<&'static MonoBindings> {
    BINDINGS.get().ok_or(MonoError::Uninitialized)
}

// TODO: the obj parameter could be enforced by the API through a trait bound
// we could implement From<c_void> for any compatible object

/// Read a field of type `T` from a Mono object at the given byte offset.
///
/// # Safety
///
/// `obj` must be a valid `MonoObject*` on a Mono-attached thread.
/// `offset` must be valid for a field of type `T`.
#[must_use]
pub unsafe fn read_field<T: Copy>(obj: *mut c_void, offset: u32) -> T {
    unsafe {
        obj.cast::<u8>()
            .add(offset as usize)
            .cast::<T>()
            .read_unaligned()
    }
}

/// Write a value of type `T` to a field of a Mono object at the given byte offset.
///
/// # Safety
///
/// Same requirements as [`read_field`].
pub unsafe fn write_field<T: Copy>(obj: *mut c_void, offset: u32, value: T) {
    unsafe {
        obj.cast::<u8>()
            .add(offset as usize)
            .cast::<T>()
            .write_unaligned(value);
    }
}
