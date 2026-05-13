//! Dynamic bindings to the Mono runtime for Windows.
//!
//! This crate attaches to a Mono runtime that is **already loaded in the current process** — it
//! does not start a new JIT domain. The intended use case is process injection into Unity games
//! and other Mono-hosted applications, where the Mono DLL is already mapped into memory before
//! any code in this crate runs.
//!
//! # Initialization
//!
//! Before using any type in this crate, call [`init`] with the name of the Mono module as it
//! appears in the host process. Common values:
//!
//! - `"mono.dll"` — Unity 2017 and earlier (legacy Mono)
//! - `"mono-2.0-bdwgc.dll"` — Unity 2018+ (Boehm GC)
//! - `"mono-2.0.dll"` — standalone Mono installations
//!
//! [`init`] uses `GetModuleHandleW` internally, so the DLL must already be loaded; it does not
//! call `LoadLibrary`.
//!
//! # Threading model
//!
//! Mono requires every thread that calls into the runtime to be registered with the garbage
//! collector. Use [`MonoThreadGuard::attach`] to register the current thread before making any
//! Mono API calls. The guard automatically deregisters the thread when dropped.
//!
//! All handle types (`MonoClass`, `MonoObject`, …) are `!Send + !Sync`. They are bound to the
//! thread on which they were obtained and cannot be moved to another thread without explicit
//! `unsafe` code. This mirrors the per-thread attachment requirement: a handle is only valid on
//! an attached thread, and the compiler prevents it from silently crossing that boundary.
//!
//! See [`MonoThreadGuard`] for the full contract.
//!
//! # Usage
//!
//! ```no_run
//! use mono_rt::prelude::*;
//!
//! // resolve exports from the already-loaded Mono DLL
//! mono_rt::init("mono-2.0-bdwgc.dll")?;
//!
//! // attach the current thread — keep the guard live for the duration of all Mono work
//! let _guard = unsafe { MonoThreadGuard::attach()? };
//!
//! // navigate the assembly graph
//! let image = MonoImage::find("Assembly-CSharp")?.expect("assembly not loaded");
//! let class = image.class_from_name("", "Player")?.expect("class not found");
//!
//! // enumerate all fields and print their names and types
//! for field in class.fields()? {
//!     let name = field.name()?;
//!     let kind = field.mono_type()?.and_then(|t| t.kind().ok());
//!     println!("{name}: {kind:?}");
//! }
//!
//! // read a field value directly from a live object
//! let hp_field = class.field("m_health")?.expect("field not found");
//! let offset = hp_field.offset()?;
//! // obj_ptr is a *mut c_void obtained from a previous MonoObject::as_ptr() call
//! // let hp: f32 = unsafe { mono_rt::read_field(obj_ptr, offset) };
//! # Ok::<(), MonoError>(())
//! ```

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
    MonoImageOpenStatus, MonoMethod, MonoObject, MonoString, MonoThread, MonoType, MonoVTable,
    TypeKind, Value,
};

/// Commonly used types, re-exported as a single glob import.
///
/// ```rust,no_run
/// use mono_rt::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{
        MonoArray, MonoAssembly, MonoClass, MonoClassField, MonoDomain, MonoError, MonoImage,
        MonoImageOpenStatus, MonoMethod, MonoObject, MonoString, MonoThread, MonoThreadGuard,
        MonoType, MonoVTable, Result, TypeKind, Value,
    };
}

static BINDINGS: OnceLock<MonoBindings> = OnceLock::new();

/// Resolves the Mono API by locating exports in the named module.
///
/// The module must already be loaded in the current process — this function calls
/// `GetModuleHandleW`, not `LoadLibraryW`. Call this once, as early as possible in your
/// injected code, before any thread attaches or any handle is created.
///
/// # Errors
///
/// - [`MonoError::DllNotFound`] if no module with `module_name` is currently loaded.
/// - [`MonoError::FnNotFound`] if a required export is missing from the module (unexpected for
///   standard Mono builds).
/// - [`MonoError::AlreadyInitialized`] if `init` has already been called successfully.
pub fn init(module_name: &str) -> Result<()> {
    let wide: Vec<u16> = module_name
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let module = unsafe { GetModuleHandleW(PCWSTR::from_raw(wide.as_ptr())) }
        .map_err(|_| MonoError::DllNotFound(module_name.to_owned()))?;

    let exports = MonoBindings::new(module)?;

    BINDINGS
        .set(exports)
        .map_err(|_| MonoError::AlreadyInitialized)?;
    Ok(())
}

/// Returns the resolved Mono API, or [`MonoError::Uninitialized`] if [`init`] has not been
/// called.
///
/// This is an internal function called by every Mono operation. It is not exposed publicly so
/// that callers cannot bypass the thread-attachment requirement.
pub(crate) fn api() -> Result<&'static MonoBindings> {
    BINDINGS.get().ok_or(MonoError::Uninitialized)
}

/// Reads a field of type `T` from a Mono object at the given byte offset.
///
/// Use [`MonoClassField::offset`](crate::MonoClassField::offset) to obtain the correct offset for
/// a field. The read uses `read_unaligned` because Mono's field layout does not guarantee that
/// fields are aligned to their natural boundary.
///
/// # Safety
///
/// - `obj` must be a valid, non-null `MonoObject*` whose class declares a field of type `T` at
///   `offset`.
/// - The current thread must be attached to the Mono runtime via [`MonoThreadGuard`] — the GC
///   must not relocate `obj` while this function is executing.
/// - `T` must have the same size and representation as the Mono field type (e.g. `f32` for a
///   `System.Single` field).
#[must_use]
pub unsafe fn read_field<T: Copy>(obj: *mut c_void, offset: u32) -> T {
    unsafe {
        obj.cast::<u8>()
            .add(offset as usize)
            .cast::<T>()
            .read_unaligned()
    }
}

/// Writes a value of type `T` into a field of a Mono object at the given byte offset.
///
/// The mirror of [`read_field`]. Use [`MonoClassField::offset`](crate::MonoClassField::offset) to
/// obtain the correct offset.
///
/// # Safety
///
/// Same requirements as [`read_field`]. Additionally, writing a reference-type field (e.g. a
/// field whose [`TypeKind`] is [`Class`](TypeKind::Class) or [`Object`](TypeKind::Object))
/// bypasses the GC write barrier and will cause memory corruption if the GC uses a generational
/// or incremental collection scheme. For reference fields, prefer invoking a managed setter via
/// [`MonoMethod::invoke`](crate::MonoMethod::invoke).
pub unsafe fn write_field<T: Copy>(obj: *mut c_void, offset: u32, value: T) {
    unsafe {
        obj.cast::<u8>()
            .add(offset as usize)
            .cast::<T>()
            .write_unaligned(value);
    }
}
