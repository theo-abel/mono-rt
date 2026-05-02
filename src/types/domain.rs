use super::mono_handle;
use crate::{Result, api};

mono_handle!(MonoDomain);

impl MonoDomain {
    /// Returns the root domain of the Mono runtime.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn root() -> Result<Option<Self>> {
        let ptr = api()?.get_root_domain();
        Ok(Self::from_ptr(ptr))
    }
}
