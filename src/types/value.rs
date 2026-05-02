use std::ffi::c_void;
use std::ptr;

/// A typed Mono method argument.
///
/// Use this with [`crate::MonoMethod::invoke_with`] to pass arguments without manually building
/// pointer arrays. The caller is still responsible for matching the argument types to the
/// method's actual signature - Mono does not validate types at the call site.
#[derive(Clone, Copy, Debug)]
pub enum Value {
    Bool(bool),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    /// A managed object or boxed value type pointer.
    Object(*mut c_void),
}

impl Value {
    /// Returns a raw pointer to the underlying data, suitable for inclusion in a Mono args array.
    pub(crate) fn as_arg_ptr(&self) -> *mut c_void {
        match self {
            Self::Bool(v) => ptr::from_ref(v).cast_mut().cast(),
            Self::I32(v) => ptr::from_ref(v).cast_mut().cast(),
            Self::I64(v) => ptr::from_ref(v).cast_mut().cast(),
            Self::F32(v) => ptr::from_ref(v).cast_mut().cast(),
            Self::F64(v) => ptr::from_ref(v).cast_mut().cast(),
            Self::Object(p) => *p,
        }
    }
}
