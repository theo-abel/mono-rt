use std::ffi::{CString, c_char};

use super::{MonoImage, mono_handle};
use crate::{MonoError, MonoImageOpenStatus, Result, api};

mono_handle!(MonoAssembly);

impl MonoAssembly {
    /// Registers a `MonoImage` as a fully loaded assembly in the current domain.
    ///
    /// `base_dir` is the directory hint Mono uses to resolve dependent assemblies. Pass `None`
    /// to rely on the standard assembly search path (correct for most injection scenarios).
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::NullByteInName`] if `base_dir` contains an interior null byte.
    /// Returns [`MonoError::ImageOpenFailed`] if Mono rejects the image.
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn load_from_image(image: MonoImage, base_dir: Option<&str>) -> Result<Option<Self>> {
        // dir_c must outlive dir_ptr — hoisted here so it lives for the whole function body.
        let dir_c;
        let dir_ptr: *const c_char = if let Some(s) = base_dir {
            dir_c = CString::new(s).map_err(|_| MonoError::NullByteInName)?;
            dir_c.as_ptr()
        } else {
            c"".as_ptr()
        };

        let mut status: i32 = 0;
        let ptr = api()?.assembly_load_from_full(
            image.as_ptr(),
            dir_ptr,
            std::ptr::addr_of_mut!(status),
            0,
        );

        let s = MonoImageOpenStatus::from_raw(status);
        if !s.is_ok() {
            return Err(MonoError::ImageOpenFailed(s));
        }

        if ptr.is_null() {
            return Err(MonoError::ImageOpenFailed(s));
        }

        Ok(MonoAssembly::from_ptr(ptr))
    }

    /// Unloads this assembly from the runtime.
    ///
    /// Call this during ejection after invoking the managed unload method. Using any handle
    /// derived from this assembly after `close` is undefined behavior.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn close(self) -> Result<()> {
        api()?.assembly_close(self.as_ptr());
        Ok(())
    }

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
