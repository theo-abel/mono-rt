//! Errors related to Mono.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, MonoError>;

#[derive(Debug, Error)]
pub enum MonoError {
    #[error("DLL `{0}` not found")]
    DllNotFound(String),
    #[error("Export not found in DLL: `{0}`")]
    FnNotFound(&'static str),
    #[error("Mono API already initialized! Multiple calls to `mono::init()` are not necessary.")]
    AlreadyInitialized,
    #[error("Mono API not initialized! Call `mono::init()` first.")]
    Uninitialized,
    #[error("string argument contains an interior null byte")]
    NullByteInName,
    #[error("managed exception was thrown during method invocation")]
    ManagedException(crate::MonoObject),
}
