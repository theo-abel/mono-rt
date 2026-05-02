use mono_rt::MonoError;

#[test]
fn init_with_nonexistent_dll_returns_dll_not_found() {
    let result = mono_rt::init("this_dll_definitely_does_not_exist_12345.dll");
    assert!(matches!(result, Err(MonoError::DllNotFound(_))));
}

#[test]
fn dll_not_found_error_preserves_module_name() {
    let module_name = "no_such_module_xyz_abc.dll";
    match mono_rt::init(module_name) {
        Err(MonoError::DllNotFound(name)) => assert_eq!(name, module_name),
        other => panic!("expected DllNotFound, got {other:?}"),
    }
}

#[test]
fn repeated_failed_inits_do_not_produce_already_initialized() {
    // AlreadyInitialized is only returned after a *successful* init.
    // Failed calls (DllNotFound) never set BINDINGS, so the second call is
    // also a DllNotFound rather than AlreadyInitialized.
    let r1 = mono_rt::init("fake_dll_first.dll");
    let r2 = mono_rt::init("fake_dll_second.dll");
    assert!(matches!(r1, Err(MonoError::DllNotFound(_))));
    assert!(matches!(r2, Err(MonoError::DllNotFound(_))));
}
