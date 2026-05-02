use super::{MonoDomain, MonoObject, mono_handle};
use crate::{Result, api};

mono_handle!(MonoType);

impl MonoType {
    #[must_use]
    pub fn get_object(self, domain: MonoDomain) -> Result<Option<MonoObject>> {
        let ptr = api()?.type_get_object(domain.as_ptr(), self.as_ptr());
        Ok(MonoObject::from_ptr(ptr))
    }
}
