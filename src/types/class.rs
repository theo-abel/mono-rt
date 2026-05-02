use super::{MonoClassField, MonoDomain, MonoMethod, MonoObject, MonoType, MonoVTable, mono_handle};
use crate::{MonoError, Result, api};

use std::ffi::CString;

mono_handle!(MonoClass);

impl MonoClass {
    /// Looks up a field by name on this class.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::NullByteInName`] if `name` contains an interior null byte.
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn field(self, name: &str) -> Result<Option<MonoClassField>> {
        let c_name = CString::new(name).map_err(|_| MonoError::NullByteInName)?;
        let ptr = api()?.class_get_field_from_name(self.as_ptr(), c_name.as_ptr());
        Ok(MonoClassField::from_ptr(ptr))
    }

    /// Looks up a method by name on this class.
    ///
    /// `param_count` restricts the search to a specific arity; pass `None` to match any overload.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::NullByteInName`] if `name` contains an interior null byte.
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn method(self, name: &str, param_count: Option<i32>) -> Result<Option<MonoMethod>> {
        let c_name = CString::new(name).map_err(|_| MonoError::NullByteInName)?;
        let ptr = api()?.class_get_method_from_name(
            self.as_ptr(),
            c_name.as_ptr(),
            param_count.unwrap_or(-1),
        );
        Ok(MonoMethod::from_ptr(ptr))
    }

    /// Returns the [`MonoType`] descriptor for this class.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn mono_type(self) -> Result<Option<MonoType>> {
        let ptr = api()?.class_get_type(self.as_ptr());
        Ok(MonoType::from_ptr(ptr))
    }

    /// Returns the vtable for this class in the given domain.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn vtable(self, domain: MonoDomain) -> Result<Option<MonoVTable>> {
        let ptr = api()?.class_vtable(domain.as_ptr(), self.as_ptr());
        Ok(MonoVTable::from_ptr(ptr))
    }

    /// Allocates a new uninitialized instance of this class in the given domain.
    ///
    /// The returned object is not yet constructed — call the `.ctor` method via
    /// [`MonoMethod::invoke`] to initialize it.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn new_object(self, domain: MonoDomain) -> Result<Option<MonoObject>> {
        let ptr = api()?.object_new(domain.as_ptr(), self.as_ptr());
        Ok(MonoObject::from_ptr(ptr))
    }

    /// Returns all fields declared on this class.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn fields(self) -> Result<Vec<MonoClassField>> {
        Ok(api()?
            .class_get_fields(self.as_ptr())
            .into_iter()
            .filter_map(MonoClassField::from_ptr)
            .collect())
    }

    /// Returns all methods declared on this class.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn methods(self) -> Result<Vec<MonoMethod>> {
        Ok(api()?
            .class_get_methods(self.as_ptr())
            .into_iter()
            .filter_map(MonoMethod::from_ptr)
            .collect())
    }
}
