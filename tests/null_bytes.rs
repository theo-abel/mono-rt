// CString construction is checked before api() is called in these methods, so
// NullByteInName is returned even when init() has never been called.

use std::ffi::c_void;

use mono_rt::{MonoAssembly, MonoClass, MonoDomain, MonoError, MonoImage, MonoString};

fn fake_ptr() -> *mut c_void {
    std::ptr::dangling_mut::<c_void>()
}

#[test]
fn string_new_rejects_interior_null() {
    let domain = unsafe { MonoDomain::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(
        MonoString::new(domain, "hel\0lo"),
        Err(MonoError::NullByteInName)
    ));
}

#[test]
fn string_new_accepts_empty_string_until_api_check() {
    // empty string produces a valid CString; the error shifts to Uninitialized
    let domain = unsafe { MonoDomain::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(
        MonoString::new(domain, ""),
        Err(MonoError::Uninitialized)
    ));
}

#[test]
fn image_class_from_name_rejects_null_in_namespace() {
    let image = unsafe { MonoImage::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(
        image.class_from_name("Some\0Namespace", "Player"),
        Err(MonoError::NullByteInName)
    ));
}

#[test]
fn image_class_from_name_rejects_null_in_class_name() {
    let image = unsafe { MonoImage::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(
        image.class_from_name("", "Play\0er"),
        Err(MonoError::NullByteInName)
    ));
}

#[test]
fn class_field_rejects_null_in_name() {
    let cls = unsafe { MonoClass::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(
        cls.field("m_hea\0lth"),
        Err(MonoError::NullByteInName)
    ));
}

#[test]
fn class_method_rejects_null_in_name() {
    let cls = unsafe { MonoClass::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(
        cls.method("Up\0date", None),
        Err(MonoError::NullByteInName)
    ));
}

#[test]
fn class_method_with_arity_rejects_null_in_name() {
    let cls = unsafe { MonoClass::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(
        cls.method("Take\0Damage", Some(1)),
        Err(MonoError::NullByteInName)
    ));
}

#[test]
fn assembly_load_from_image_rejects_null_in_base_dir() {
    let image = unsafe { MonoImage::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(
        MonoAssembly::load_from_image(image, Some("path\0null")),
        Err(MonoError::NullByteInName)
    ));
}

#[test]
fn domain_open_assembly_rejects_null_in_path() {
    let domain = unsafe { MonoDomain::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(
        domain.open_assembly("path/to\0/file.dll"),
        Err(MonoError::NullByteInName)
    ));
}
