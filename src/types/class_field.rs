use std::ffi::CStr;

use super::{MonoType, MonoVTable, mono_handle};
use crate::{Result, api};

mono_handle!(MonoClassField);

impl MonoClassField {
    /// Returns the byte offset of this field within its declaring class.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn offset(self) -> Result<u32> {
        Ok(api()?.field_get_offset(self.as_ptr()))
    }

    /// Returns the name of this field as reported by the Mono runtime.
    ///
    /// The returned string is copied out of Mono's metadata and is safe to use beyond the
    /// lifetime of the runtime handle.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn name(self) -> Result<String> {
        let ptr = api()?.field_get_name(self.as_ptr());
        if ptr.is_null() {
            return Ok(String::new());
        }
        Ok(unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned())
    }

    /// Returns the [`MonoType`] descriptor for this field.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn mono_type(self) -> Result<Option<MonoType>> {
        let ptr = api()?.field_get_type(self.as_ptr());
        Ok(MonoType::from_ptr(ptr))
    }

    /// Reads the static field value into the `out` buffer via the given vtable.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    ///
    /// # Safety
    ///
    /// `out` must point to valid memory sized for the field type.
    pub unsafe fn static_value(self, vtable: MonoVTable, out: *mut c_void) -> Result<()> {
        api()?.field_static_get_value(vtable.as_ptr(), self.as_ptr(), out);
        Ok(())
    }
}
