use super::{MonoClassField, MonoDomain, MonoMethod, MonoType, MonoVTable, mono_handle};
use crate::{Result, api};

use std::ffi::CString;

mono_handle!(MonoClass);

// TODO: handle all failing cstring properly

impl MonoClass {
    #[must_use]
    pub fn get_field(self, name: &str) -> Result<Option<MonoClassField>> {
        let c_name = CString::new(name).ok().expect("CString failed");
        let ptr = api()?.class_get_field_from_name(self.as_ptr(), c_name.as_ptr());
        Ok(MonoClassField::from_ptr(ptr))
    }

    #[must_use]
    pub fn get_method(self, name: &str, param_count: i32) -> Result<Option<MonoMethod>> {
        let c_name = CString::new(name).ok().expect("CString failed");
        let ptr = api()?.class_get_method_from_name(self.as_ptr(), c_name.as_ptr(), param_count);
        Ok(MonoMethod::from_ptr(ptr))
    }

    #[must_use]
    pub fn get_type(self) -> Result<Option<MonoType>> {
        let ptr = api()?.class_get_type(self.as_ptr());
        Ok(MonoType::from_ptr(ptr))
    }

    #[must_use]
    pub fn get_vtable(self, domain: MonoDomain) -> Result<Option<MonoVTable>> {
        let ptr = api()?.class_vtable(domain.as_ptr(), self.as_ptr());
        Ok(MonoVTable::from_ptr(ptr))
    }
}
