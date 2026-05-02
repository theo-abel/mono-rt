//! RAII guard for Mono thread attachment.
//!
//! Mono requires every thread that interacts with managed objects to register itself with the
//! runtime via `mono_thread_attach` before making any API calls, and to unregister via
//! `mono_thread_detach` before the thread exits. Failing to detach leaks the thread's entry in
//! Mono's internal thread table and prevents the garbage collector from reclaiming associated
//! resources.
//!
//! [`MonoThreadGuard`] encapsulates this lifecycle: constructing it attaches the current thread,
//! and dropping it detaches it. Because [`MonoThreadGuard`] is [`!Send`][std::marker::Send], the
//! compiler guarantees that drop happens on the same thread as construction, which is required by
//! Mono's attach/detach contract.

use std::ffi::c_void;
use std::marker::PhantomData;

use super::{Result, api};

/// A RAII guard that keeps the current thread attached to the Mono runtime.
///
/// Constructing this guard calls `mono_thread_attach`, registering the thread with the Mono GC
/// and enabling the use of all Mono handle types. Dropping it calls `mono_thread_detach`,
/// releasing the thread's entry in the runtime.
///
/// # `!Send + !Sync`
///
/// This type deliberately does not implement [`Send`] or [`Sync`]. `mono_thread_detach` must be
/// called from the same operating-system thread that called `mono_thread_attach`. If the guard
/// were moved to another thread and dropped there, it would detach the wrong thread, leaving the
/// original thread permanently registered and causing the detaching thread to corrupt Mono's
/// internal bookkeeping.
///
/// The `!Send` bound also creates a natural pairing with the handle types in this crate: since
/// all Mono handles are `!Send + !Sync`, they cannot escape the thread on which they were
/// obtained, and that thread is guaranteed to hold a live guard.
///
/// # Panics
///
/// The `Drop` implementation does not panic. If the Mono API is unavailable at drop time
/// (which cannot happen under normal use since the guard can only be constructed after a
/// successful [`init`](crate::init)), a warning is emitted via [`tracing`] and the detach is
/// skipped.
#[must_use = "dropping this guard immediately detaches the thread from Mono"]
pub struct MonoThreadGuard {
    thread_ptr: *mut c_void,
    // makes the type !Send + !Sync, enforcing that drop happens on the attaching thread
    _marker: PhantomData<*mut ()>,
}

impl MonoThreadGuard {
    /// Attaches the current thread to the Mono runtime.
    ///
    /// This registers the thread with Mono's garbage collector. Any Mono handle type obtained
    /// after this call is safe to use on the current thread for as long as this guard is live.
    ///
    /// Prefer creating one guard per thread at the top of the thread's entry point and keeping it
    /// alive for the entire duration of Mono usage on that thread.
    ///
    /// # Errors
    ///
    /// Returns [`MonoError::Uninitialized`](crate::MonoError::Uninitialized) if [`init`](crate::init)
    /// has not been called yet.
    ///
    /// # Safety
    ///
    /// The returned guard must be dropped on the same thread that called `attach`. Dropping it on
    /// a different thread would call `mono_thread_detach` for the wrong thread, corrupting Mono's
    /// internal thread registry.
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
