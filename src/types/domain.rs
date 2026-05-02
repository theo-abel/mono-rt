use std::ffi::CString;

use super::{MonoAssembly, mono_handle};
use crate::{MonoError, Result, api};

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

    /// Opens (loads) an assembly from the given file path into this domain.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::NullByteInName`] if `path` contains an interior null byte.
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn open_assembly(self, path: &str) -> Result<Option<MonoAssembly>> {
        let c_path = CString::new(path).map_err(|_| MonoError::NullByteInName)?;
        let ptr = api()?.domain_assembly_open(self.as_ptr(), c_path.as_ptr());
        Ok(MonoAssembly::from_ptr(ptr))
    }
}
