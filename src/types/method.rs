use super::{MonoObject, mono_handle};
use crate::{Result, api};

use std::ptr;

mono_handle!(MonoMethod);

impl MonoMethod {
    /// Invokes the method on `obj` (null for static) with arguments.
    ///
    /// Returns `Ok(Some(result))` on success, `Ok(None)` when the method throws a managed
    /// exception or returns void, and `Err(MonoAv)` when a hardware access violation is
    /// caught inside the SEH shim (e.g. JIT pages freed during scene teardown).
    ///
    /// # Errors
    ///
    /// Returns `Err(MonoAv)` when the SEH shim catches a hardware access violation.
    ///
    /// # Safety
    ///
    /// `obj` and `args` must be valid Mono objects/arguments matching the method signature.
    pub unsafe fn invoke(
        self,
        obj: *mut c_void,
        args: *mut *mut c_void,
    ) -> Result<Option<MonoObject>> {
        let mut exc = ptr::null_mut::<c_void>();
        let result = api()?.runtime_invoke(self.as_ptr(), obj, args, ptr::addr_of_mut!(exc))?;
        if exc.is_null() {
            Ok(MonoObject::from_ptr(result))
        } else {
            Ok(None)
        }
    }
}
