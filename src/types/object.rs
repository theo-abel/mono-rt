use super::mono_handle;
use crate::{Result, api};

mono_handle!(MonoObject);

impl MonoObject {
    /// Unboxes this object and returns a raw pointer to its value-type data.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn unbox(self) -> Result<*mut c_void> {
        Ok(api()?.object_unbox(self.as_ptr()))
    }
}
