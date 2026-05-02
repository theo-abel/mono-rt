use super::{MonoDomain, MonoObject, mono_handle};
use crate::{Result, api};

mono_handle!(MonoType);

impl MonoType {
    /// Returns a [`MonoObject`] wrapping this type descriptor in the given domain.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn object(self, domain: MonoDomain) -> Result<Option<MonoObject>> {
        let ptr = api()?.type_get_object(domain.as_ptr(), self.as_ptr());
        Ok(MonoObject::from_ptr(ptr))
    }
}
