//! Errors related to Mono.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, MonoError>;

#[derive(Debug, Error)]
pub enum MonoError {
    #[error("DLL `{0}` not found")]
    DllNotFound(String),
    #[error("Export not found in DLL: `{0}`")]
    FnNotFound(&'static str),
    #[error("Mono API already initialized! Multiple calls to `mono::init()` are not necessary.")]
    AlreadyInitialized,
    #[error("Mono API not initialized! Call `mono::init()` first.")]
    Uninitialized,
    #[error("string argument contains an interior null byte")]
    NullByteInName,
    #[error("managed exception was thrown during method invocation")]
    ManagedException(crate::MonoObject),
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::MonoError;
    use crate::MonoObject;

    #[test]
    fn dll_not_found_display() {
        let e = MonoError::DllNotFound("mono.dll".to_owned());
        assert_eq!(e.to_string(), "DLL `mono.dll` not found");
    }

    #[test]
    fn fn_not_found_display() {
        let e = MonoError::FnNotFound("mono_get_root_domain");
        assert_eq!(
            e.to_string(),
            "Export not found in DLL: `mono_get_root_domain`"
        );
    }

    #[test]
    fn already_initialized_display() {
        let e = MonoError::AlreadyInitialized;
        assert!(e.to_string().contains("already initialized"));
    }

    #[test]
    fn uninitialized_display() {
        let e = MonoError::Uninitialized;
        assert!(e.to_string().contains("not initialized"));
    }

    #[test]
    fn null_byte_in_name_display() {
        let e = MonoError::NullByteInName;
        assert_eq!(
            e.to_string(),
            "string argument contains an interior null byte"
        );
    }

    #[test]
    fn managed_exception_display() {
        let obj = unsafe { MonoObject::from_ptr_unchecked(std::ptr::dangling_mut()) };
        let e = MonoError::ManagedException(obj);
        assert_eq!(
            e.to_string(),
            "managed exception was thrown during method invocation"
        );
    }

    #[test]
    fn implements_std_error_trait() {
        let e: Box<dyn Error> = Box::new(MonoError::Uninitialized);
        assert!(e.source().is_none());
    }

    #[test]
    fn dll_not_found_debug_contains_variant_name() {
        let e = MonoError::DllNotFound("x.dll".to_owned());
        assert!(format!("{e:?}").contains("DllNotFound"));
    }

    #[test]
    fn fn_not_found_debug_contains_variant_name() {
        let e = MonoError::FnNotFound("some_export");
        assert!(format!("{e:?}").contains("FnNotFound"));
    }
}
