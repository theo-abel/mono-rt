use super::mono_handle;
use crate::{Result, api};

mono_handle!(MonoObject);

impl MonoObject {
    #[must_use]
    pub fn unbox(self) -> Result<*mut c_void> {
        Ok(api()?.object_unbox(self.as_ptr()))
    }
}
