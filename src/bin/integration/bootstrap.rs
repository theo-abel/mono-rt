use std::ffi::c_char;
use std::ffi::c_void;
use std::fmt;
use std::os::windows::ffi::OsStrExt as _;
use std::path::Path;

use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows::core::PCWSTR;

type MonoJitInitFn = unsafe extern "C" fn(*const c_char) -> *mut c_void;
type RawFn = unsafe extern "system" fn() -> isize;

#[derive(Debug)]
pub enum BootstrapError {
    LoadLibrary(String),
    ResolveFn(String),
    MonoInit(mono_rt::MonoError),
}

impl fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoadLibrary(msg) => write!(f, "failed to load Mono DLL: {msg}"),
            Self::ResolveFn(msg) => write!(f, "failed to resolve mono_jit_init: {msg}"),
            Self::MonoInit(e) => write!(f, "mono_rt::init failed: {e}"),
        }
    }
}

impl std::error::Error for BootstrapError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MonoInit(e) => Some(e),
            _ => None,
        }
    }
}

fn load_dll(path: &Path) -> Result<HMODULE, BootstrapError> {
    let wide: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    unsafe { LoadLibraryW(PCWSTR(wide.as_ptr())) }
        .map_err(|e| BootstrapError::LoadLibrary(e.to_string()))
}

fn resolve_jit_init(module: HMODULE) -> Result<MonoJitInitFn, BootstrapError> {
    let raw = unsafe { GetProcAddress(module, windows::core::s!("mono_jit_init")) };
    let func = raw
        .ok_or_else(|| BootstrapError::ResolveFn("mono_jit_init not exported by DLL".to_owned()))?;
    // Safety: mono_jit_init is a well-known Mono embedding API symbol with this exact signature.
    Ok(unsafe { std::mem::transmute::<RawFn, MonoJitInitFn>(func) })
}

fn call_jit_init(f: MonoJitInitFn) {
    // Safety: f is a valid function pointer resolved from the loaded Mono DLL.
    unsafe { f(c"mono-rt-tests".as_ptr()) };
}

fn dll_stem(path: &Path) -> String {
    path.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}

/// Loads `dll_path`, calls `mono_jit_init` to create the root domain, then resolves the
/// `mono_rt` bindings so the crate's public API is ready to use.
///
/// # Errors
///
/// - [`BootstrapError::LoadLibrary`] if `LoadLibraryW` fails.
/// - [`BootstrapError::ResolveFn`] if `mono_jit_init` is not exported by the DLL.
/// - [`BootstrapError::MonoInit`] if `mono_rt::init` returns an error.
pub fn bootstrap(dll_path: &Path) -> Result<(), BootstrapError> {
    let module = load_dll(dll_path)?;
    let jit_init = resolve_jit_init(module)?;
    call_jit_init(jit_init);
    mono_rt::init(&dll_stem(dll_path)).map_err(BootstrapError::MonoInit)
}
