//! Mono type wrappers and utilities.
//!
//! Provides safe, transparent wrappers around Mono pointer types.

mod array;
mod assembly;
mod class;
mod class_field;
mod domain;
mod image;
mod method;
mod mono_type;
mod object;
mod string;
mod thread;
mod vtable;

pub use array::MonoArray;
pub use assembly::MonoAssembly;
pub use class::MonoClass;
pub use class_field::MonoClassField;
pub use domain::MonoDomain;
pub use image::MonoImage;
pub use method::MonoMethod;
pub use mono_type::MonoType;
pub use object::MonoObject;
pub use string::MonoString;
pub use thread::MonoThread;
pub use vtable::MonoVTable;

use std::ffi::c_void;

pub type MonoFunc = unsafe extern "C" fn(data: *mut c_void, user_data: *mut c_void);

/// Transparent wrapper around a Mono pointer type, ensuring non-null and providing safe conversions.
/// The inner pointer is opaque and should only be accessed through safe wrapper methods on each type.
macro_rules! mono_handle {
    ($name:ident) => {
        use std::{ffi::c_void, ptr::NonNull};

        #[repr(transparent)]
        #[derive(Clone, Copy)]
        pub struct $name(NonNull<c_void>);

        impl $name {
            #[must_use]
            pub fn as_ptr(self) -> *mut c_void {
                self.0.as_ptr()
            }

            #[must_use]
            pub fn from_ptr(ptr: *mut c_void) -> Option<Self> {
                NonNull::new(ptr).map(Self)
            }

            /// # Safety
            ///
            /// `ptr` must be valid and non-null on a thread attached to Mono.
            #[must_use]
            pub unsafe fn from_ptr_unchecked(ptr: *mut c_void) -> Self {
                Self(unsafe { NonNull::new_unchecked(ptr) })
            }
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}
    };
}
pub(crate) use mono_handle;
