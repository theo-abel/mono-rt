use std::ffi::{CStr, CString};
use std::ptr;

use super::{MonoClass, mono_handle};
use crate::{MonoError, Result, api};

struct FindContext<'a> {
    target_name: &'a str,
    result: Option<MonoImage>,
    api_failed: bool,
}

mono_handle!(MonoImage);

impl MonoImage {
    /// Finds a loaded assembly image by name.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn find(name: &str) -> Result<Option<Self>> {
        let mut ctx = FindContext {
            target_name: name,
            result: None,
            api_failed: false,
        };
        api()?.assembly_foreach(find_image_callback, ptr::addr_of_mut!(ctx).cast());
        if ctx.api_failed {
            return Err(MonoError::Uninitialized);
        }
        Ok(ctx.result)
    }

    /// Resolves a class in this image by namespace and name.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::NullByteInName`] if `namespace` or `name` contain an interior null byte.
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn class_from_name(self, namespace: &str, name: &str) -> Result<Option<MonoClass>> {
        let ns = CString::new(namespace).map_err(|_| MonoError::NullByteInName)?;
        let nm = CString::new(name).map_err(|_| MonoError::NullByteInName)?;
        let ptr = api()?.class_from_name(self.as_ptr(), ns.as_ptr(), nm.as_ptr());
        Ok(MonoClass::from_ptr(ptr))
    }
}

/// Callback for `mono_assembly_foreach` to find an image by name.
///
/// # Safety
///
/// `assembly` must be a valid `MonoAssembly*` and `user_data` must be a valid pointer to a
/// `FindContext`, both on a Mono-attached thread.
unsafe extern "C" fn find_image_callback(assembly: *mut c_void, user_data: *mut c_void) {
    let Ok(api) = api() else {
        let ctx = unsafe { &mut *user_data.cast::<FindContext<'_>>() };
        ctx.api_failed = true;
        return;
    };

    let ctx = unsafe { &mut *user_data.cast::<FindContext<'_>>() };
    if ctx.result.is_some() {
        return;
    }

    let image = api.assembly_get_image(assembly);
    if image.is_null() {
        return;
    }

    let name_ptr = api.image_get_name(image);
    if name_ptr.is_null() {
        return;
    }

    let name = unsafe { CStr::from_ptr(name_ptr) }.to_str().unwrap_or("");
    if name == ctx.target_name {
        ctx.result = Some(unsafe { MonoImage::from_ptr_unchecked(image) });
    }
}
