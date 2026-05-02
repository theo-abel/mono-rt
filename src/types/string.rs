use super::mono_handle;
use crate::{Result, api};

use std::ffi::CStr;

mono_handle!(MonoString);

impl MonoString {
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
