use super::mono_handle;
use crate::{Result, api};

mono_handle!(MonoArray);

impl MonoArray {
    /// Returns the number of elements in this array.
    ///
    /// # Errors
    ///
    /// Returns [`crate::MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn len(self) -> Result<usize> {
        Ok(api()?.array_length(self.as_ptr()))
    }

    /// Returns `true` if the array contains no elements.
    ///
    /// # Errors
    ///
    /// Returns [`crate::MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn is_empty(self) -> Result<bool> {
        Ok(self.len()? == 0)
    }

    /// Returns a pointer to the element at `index`, assuming each element is `size` bytes.
    ///
    /// # Errors
    ///
    /// Returns [`crate::MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn addr(self, size: i32, index: usize) -> Result<*mut c_void> {
        Ok(api()?.array_addr_with_size(self.as_ptr(), size, index))
    }
}
