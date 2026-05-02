use super::{MonoVTable, mono_handle};
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
