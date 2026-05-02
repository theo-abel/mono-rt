use std::ffi::CStr;

use super::{MonoObject, Value, mono_handle};
use crate::{MonoError, Result, api};

use std::ptr;

mono_handle!(MonoMethod);

impl MonoMethod {
    /// Returns the name of this method as reported by the Mono runtime.
    ///
    /// The returned string is copied out of Mono's metadata and is safe to use beyond the
    /// lifetime of the runtime handle.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    pub fn name(self) -> Result<String> {
        let ptr = api()?.method_get_name(self.as_ptr());
        if ptr.is_null() {
            return Ok(String::new());
        }
        Ok(unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned())
    }

    /// Invokes the method on `obj` (null for static methods) with the given arguments.
    ///
    /// Returns `Ok(Some(result))` on success, `Ok(None)` when the method returns void, and
    /// `Err(MonoError::ManagedException(exc))` when the invocation throws a managed exception.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::ManagedException`] when a managed exception is thrown.
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
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
        let result =
            api()?.runtime_invoke(self.as_ptr(), obj, args, ptr::addr_of_mut!(exc));

        if !exc.is_null() {
            let exc_obj = unsafe { MonoObject::from_ptr_unchecked(exc) };
            return Err(MonoError::ManagedException(exc_obj));
        }

        Ok(MonoObject::from_ptr(result))
    }

    /// Typed variant of [`invoke`](Self::invoke): builds the args array from `args` automatically.
    ///
    /// Prefer this over `invoke` when argument types are known at compile time. The caller is still
    /// responsible for matching the `Value` variants to the method's actual parameter types.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::ManagedException`] when a managed exception is thrown.
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    ///
    /// # Safety
    ///
    /// `obj` must be a valid Mono object pointer (or null for static methods). Each [`Value`]
    /// in `args` must correspond to the correct parameter type expected by the method.
    pub unsafe fn invoke_with(
        self,
        obj: *mut c_void,
        args: &[Value],
    ) -> Result<Option<MonoObject>> {
        let mut ptrs: Vec<*mut c_void> = args.iter().map(Value::as_arg_ptr).collect();
        let args_ptr = if ptrs.is_empty() {
            std::ptr::null_mut()
        } else {
            ptrs.as_mut_ptr()
        };
        unsafe { self.invoke(obj, args_ptr) }
    }
}
