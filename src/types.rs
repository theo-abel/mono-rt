//! Mono type wrappers and utilities.
//!
//! Provides safe, transparent wrappers around Mono pointer types.

mod array;
mod assembly;
mod class;
mod class_field;
mod domain;
mod image;
mod image_open_status;
mod method;
mod mono_type;
mod object;
mod string;
mod thread;
mod type_kind;
mod value;
mod vtable;

pub use array::MonoArray;
pub use assembly::MonoAssembly;
pub use class::MonoClass;
pub use class_field::MonoClassField;
pub use domain::MonoDomain;
pub use image::MonoImage;
pub use image_open_status::MonoImageOpenStatus;
pub use method::MonoMethod;
pub use mono_type::MonoType;
pub use object::MonoObject;
pub use string::MonoString;
pub use thread::MonoThread;
pub use type_kind::TypeKind;
pub use value::Value;
pub use vtable::MonoVTable;

use std::ffi::c_void;

pub type MonoFunc = unsafe extern "C" fn(data: *mut c_void, user_data: *mut c_void);

/// Generates a `!Send + !Sync` transparent wrapper around an opaque Mono pointer.
///
/// All handle types produced by this macro share the same threading contract, documented
/// on the generated struct below.
macro_rules! mono_handle {
    ($name:ident) => {
        use std::{ffi::c_void, ptr::NonNull};

        /// An opaque handle to a Mono runtime object.
        ///
        /// # Thread safety
        ///
        /// This type is intentionally `!Send + !Sync`. Every thread that reads or writes
        /// Mono objects must first be registered with the runtime via
        /// [`crate::MonoThreadGuard::attach`]. Using a handle on an unregistered thread is
        /// undefined behavior — Mono's garbage collector and internal bookkeeping assume all
        /// active threads are known to the runtime.
        ///
        /// Handles are therefore bound to the thread on which they were obtained. The
        /// compiler enforces this: a handle cannot be moved to another thread without
        /// explicit `unsafe` code.
        ///
        /// If you need to transfer a handle across thread boundaries and you can guarantee
        /// that *both* threads are attached to the runtime for the entire duration of use,
        /// you can opt in manually on your wrapper type:
        ///
        /// ```rust,ignore
        /// struct MyComponent {
        ///     class: mono_rt::MonoClass,
        /// }
        ///
        /// // SAFETY: `class` is only accessed while the calling thread holds a
        /// // `MonoThreadGuard`, ensuring it is registered with the Mono runtime.
        /// unsafe impl Send for MyComponent {}
        /// unsafe impl Sync for MyComponent {}
        /// ```
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug)]
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
            /// `ptr` must be non-null and valid on a thread attached to the Mono runtime.
            #[must_use]
            pub unsafe fn from_ptr_unchecked(ptr: *mut c_void) -> Self {
                Self(unsafe { NonNull::new_unchecked(ptr) })
            }
        }
    };
}
pub(crate) use mono_handle;
