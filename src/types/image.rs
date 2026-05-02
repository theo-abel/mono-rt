use std::ffi::{CStr, CString};
use std::ptr;

use super::{MonoClass, mono_handle};
use crate::{Result, api};

struct FindContext<'a> {
    target_name: &'a str,
    result: Option<MonoImage>,
}

mono_handle!(MonoImage);

impl MonoImage {
    /// Finds a loaded assembly image by name.
    #[must_use]
    pub fn find<T: AsRef<str>>(name: T) -> Result<Option<Self>> {
        let mut ctx = FindContext {
            target_name: name.as_ref(),
            result: None,
        };
        api()?.assembly_foreach(find_image_callback, ptr::addr_of_mut!(ctx).cast());
        Ok(ctx.result)
    }

    /// Resolves a class in this image.
    #[must_use]
    pub fn get_class_from_name<T: AsRef<str>>(
        self,
        namespace: T,
        name: T,
    ) -> Result<Option<MonoClass>> {
        // TODO: handle cstrings results !! we don't want panic here
        let ns = CString::new(namespace.as_ref()).expect("CString failed");
        let nm = CString::new(name.as_ref()).expect("CString failed");
        let ptr = api()?.class_from_name(self.as_ptr(), ns.as_ptr(), nm.as_ptr());
        Ok(MonoClass::from_ptr(ptr))
    }
}

/// Callback for `mono_assembly_foreach` to find an image by name. Stores the result in the provided context.
///
/// # Safety
///
/// `assembly` is a valid `MonoAssembly*` on a Mono-attached thread.
/// `user_data` is a valid pointer to a `FindContext`.
unsafe extern "C" fn find_image_callback(assembly: *mut c_void, user_data: *mut c_void) {
    // TODO: find a way to avoid returning early on API access failure, maybe by caching the API in a static or something? :c
    let api = match api() {
        Ok(api) => api,
        Err(_) => return, // we can't access the API, just skip processing
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
