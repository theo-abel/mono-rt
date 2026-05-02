//! A guard that ensures the current thread is attached to the Mono runtime while it is alive.
//!
//! This is necessary for any thread that interacts with Mono objects or functions, as Mono requires threads to be explicitly attached and detached.

use std::ffi::c_void;
use std::marker::PhantomData;

use super::{Result, api};

/// Dropping this guard immediately detaches the current thread from the Mono runtime.
#[must_use = "dropping this guard immediately detaches the thread from Mono"]
pub struct MonoThreadGuard {
    thread_ptr: *mut c_void,
    _marker: PhantomData<*mut ()>,
}

impl MonoThreadGuard {
    /// Attaches the current thread to the Mono runtime.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`] if the Mono API has not been initialized.
    ///
    /// # Safety
    ///
    /// Must be called on the thread that will own all Mono interactions.
    /// The returned guard must be dropped on that same thread.
    pub unsafe fn attach() -> Result<Self> {
        let domain = api()?.get_root_domain();
        let thread_ptr = api()?.thread_attach(domain);

        Ok(Self {
            thread_ptr,
            _marker: PhantomData,
        })
    }
}

impl Drop for MonoThreadGuard {
    fn drop(&mut self) {
        match api() {
            Ok(api) => api.thread_detach(self.thread_ptr),
            Err(e) => tracing::warn!("could not detach Mono thread on drop: {e}"),
        }
    }
}
