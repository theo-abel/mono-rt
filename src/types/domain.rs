use super::mono_handle;
use crate::{Result, api};

mono_handle!(MonoDomain);

impl MonoDomain {
    #[must_use]
    pub fn get_root() -> Result<Option<Self>> {
        let ptr = api()?.get_root_domain();
        Ok(Self::from_ptr(ptr))
    }
}
