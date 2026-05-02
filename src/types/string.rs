use std::ffi::{CStr, CString};

use super::{MonoDomain, mono_handle};
use crate::{MonoError, Result, api};

mono_handle!(MonoString);

impl MonoString {
    /// Creates a new Mono string from a Rust string slice in the given domain.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::NullByteInName`] if `text` contains an interior null byte.
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn new(domain: MonoDomain, text: &str) -> Result<Option<Self>> {
        let c_text = CString::new(text).map_err(|_| MonoError::NullByteInName)?;
        let ptr = api()?.string_new(domain.as_ptr(), c_text.as_ptr());
        Ok(Self::from_ptr(ptr))
    }

    /// Converts this Mono string to a Rust `String`, replacing invalid UTF-8 sequences.
    ///
    /// Returns an empty string if the underlying pointer is null after conversion.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn to_string_lossy(self) -> Result<String> {
        let raw = api()?.string_to_utf8(self.as_ptr().cast());
        if raw.is_null() {
            return Ok(String::new());
        }

        let result = unsafe { CStr::from_ptr(raw) }
            .to_string_lossy()
            .into_owned();

        api()?.free(raw.cast());

        Ok(result)
    }
}
