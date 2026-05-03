mod bootstrap;
mod harness;
mod tests;

use std::fmt;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mono_dll = arg(&args, 1, "mono-dll");
    let fixture_dll = arg(&args, 2, "fixture-dll");

    bootstrap::bootstrap(&mono_dll).unwrap_or_else(|e| fatal(e));

    // Safety: bootstrap guarantees mono_rt::init succeeded and the Mono JIT is live.
    let _guard = unsafe { mono_rt::MonoThreadGuard::attach() }
        .expect("thread attach must succeed after bootstrap");

    let fixture_str = fixture_dll.to_string_lossy().into_owned();
    let ctx = tests::setup_context(&fixture_str).unwrap_or_else(|e| fatal(e));

    let mut h = harness::Harness::new();
    tests::register_all(&mut h, &ctx);

    if h.finish() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

fn arg(args: &[String], index: usize, name: &str) -> PathBuf {
    match args.get(index) {
        Some(v) => PathBuf::from(v),
        None => fatal(format!("missing argument: {name}")),
    }
}

fn fatal(msg: impl fmt::Display) -> ! {
    eprintln!("fatal: {msg}");
    std::process::exit(1);
}
