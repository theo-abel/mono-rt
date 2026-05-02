use super::{MonoVTable, mono_handle};
use crate::{Result, api};

mono_handle!(MonoClassField);

impl MonoClassField {
    #[must_use]
    pub fn get_offset(self) -> Result<u32> {
        Ok(api()?.field_get_offset(self.as_ptr()))
    }

    /// Reads a static field value into the `out` buffer.
    ///
    /// # Safety
    ///
    /// `out` must point to valid memory for the field type.
    pub unsafe fn get_static_value(self, vtable: MonoVTable, out: *mut c_void) -> Result<()> {
        // TODO: may return a boxed value instead of writing to out ?
        api()?.field_static_get_value(vtable.as_ptr(), self.as_ptr(), out);
        Ok(())
    }
}
