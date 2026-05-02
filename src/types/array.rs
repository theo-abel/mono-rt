use super::mono_handle;
use crate::{Result, api};

mono_handle!(MonoArray);

impl MonoArray {
    #[must_use]
    pub fn len(self) -> Result<usize> {
        Ok(api()?.array_length(self.as_ptr()))
    }

    #[must_use]
    pub fn is_empty(self) -> Result<bool> {
        Ok(self.len()? == 0)
    }

    /// Returns a pointer to the element at `index`, assuming elements are `size` bytes.
    #[must_use]
    pub fn get_addr(self, size: i32, index: usize) -> Result<*mut c_void> {
        Ok(api()?.array_addr_with_size(self.as_ptr(), size, index))
    }
}
