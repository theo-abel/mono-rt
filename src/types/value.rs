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

#[cfg(test)]
mod tests {
    use std::ffi::c_void;

    use super::Value;

    #[test]
    fn value_is_copy() {
        let a = Value::I32(42);
        let b = a;
        // both remain usable after a copy
        let _ = a;
        let _ = b;
    }

    #[test]
    fn bool_true_arg_ptr_reads_back() {
        let v = Value::Bool(true);
        let read = unsafe { v.as_arg_ptr().cast::<bool>().read() };
        assert!(read);
    }

    #[test]
    fn bool_false_arg_ptr_reads_back() {
        let v = Value::Bool(false);
        let read = unsafe { v.as_arg_ptr().cast::<bool>().read() };
        assert!(!read);
    }

    #[test]
    fn i32_arg_ptr_reads_back() {
        let v = Value::I32(-99_999);
        let read = unsafe { v.as_arg_ptr().cast::<i32>().read() };
        assert_eq!(read, -99_999);
    }

    #[test]
    fn i64_arg_ptr_reads_back() {
        let v = Value::I64(i64::MAX);
        let read = unsafe { v.as_arg_ptr().cast::<i64>().read() };
        assert_eq!(read, i64::MAX);
    }

    #[test]
    fn f32_arg_ptr_reads_back() {
        let v = Value::F32(1.5_f32);
        let read = unsafe { v.as_arg_ptr().cast::<f32>().read() };
        assert!((read - 1.5_f32).abs() < f32::EPSILON);
    }

    #[test]
    fn f64_arg_ptr_reads_back() {
        let v = Value::F64(std::f64::consts::PI);
        let read = unsafe { v.as_arg_ptr().cast::<f64>().read() };
        assert!((read - std::f64::consts::PI).abs() < f64::EPSILON);
    }

    #[test]
    fn object_arg_ptr_returns_stored_pointer() {
        let raw: *mut c_void = 0xDEAD_BEEF_usize as *mut _;
        let v = Value::Object(raw);
        assert_eq!(v.as_arg_ptr(), raw);
    }

    #[test]
    fn object_null_arg_ptr_is_null() {
        let v = Value::Object(std::ptr::null_mut());
        assert!(v.as_arg_ptr().is_null());
    }

    #[test]
    fn debug_output_includes_variant_names() {
        assert!(format!("{:?}", Value::Bool(true)).contains("Bool"));
        assert!(format!("{:?}", Value::I32(0)).contains("I32"));
        assert!(format!("{:?}", Value::I64(0)).contains("I64"));
        assert!(format!("{:?}", Value::F32(0.0)).contains("F32"));
        assert!(format!("{:?}", Value::F64(0.0)).contains("F64"));
        assert!(format!("{:?}", Value::Object(std::ptr::null_mut())).contains("Object"));
    }
}
