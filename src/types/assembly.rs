use super::{MonoImage, mono_handle};
use crate::{Result, api};

mono_handle!(MonoAssembly);

impl MonoAssembly {
    /// Returns the metadata image for this assembly.
    ///
    /// # Errors
    ///
    /// Returns [`crate::MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn image(self) -> Result<Option<MonoImage>> {
        let ptr = api()?.assembly_get_image(self.as_ptr());
        Ok(MonoImage::from_ptr(ptr))
    }
}
