use super::{MonoClass, mono_handle};
use crate::{Result, api};

mono_handle!(MonoObject);

impl MonoObject {
    /// Returns the `MonoClass` of this object.
    ///
    /// Primarily useful for identifying exception types after a failed method invocation.
    ///
    /// # Errors
    ///
    /// Returns [`crate::MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn get_class(self) -> Result<Option<MonoClass>> {
        let ptr = api()?.object_get_class(self.as_ptr());
        Ok(MonoClass::from_ptr(ptr))
    }

    /// Unboxes this object and returns a raw pointer to its value-type data.
    ///
    /// # Errors
    ///
    /// Returns [`crate::MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn unbox(self) -> Result<*mut c_void> {
        Ok(api()?.object_unbox(self.as_ptr()))
    }
}
