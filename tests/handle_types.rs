use std::ffi::c_void;
use std::ptr;

use mono_rt::{
    MonoArray, MonoAssembly, MonoClass, MonoClassField, MonoDomain, MonoImage, MonoMethod,
    MonoObject, MonoString, MonoThread, MonoType, MonoVTable,
};

fn fake_ptr() -> *mut c_void {
    // a stable non-null sentinel — never dereferenced in these tests
    ptr::dangling_mut::<c_void>()
}

#[test]
fn from_ptr_null_returns_none_for_all_types() {
    assert!(MonoArray::from_ptr(ptr::null_mut()).is_none());
    assert!(MonoAssembly::from_ptr(ptr::null_mut()).is_none());
    assert!(MonoClass::from_ptr(ptr::null_mut()).is_none());
    assert!(MonoClassField::from_ptr(ptr::null_mut()).is_none());
    assert!(MonoDomain::from_ptr(ptr::null_mut()).is_none());
    assert!(MonoImage::from_ptr(ptr::null_mut()).is_none());
    assert!(MonoMethod::from_ptr(ptr::null_mut()).is_none());
    assert!(MonoObject::from_ptr(ptr::null_mut()).is_none());
    assert!(MonoString::from_ptr(ptr::null_mut()).is_none());
    assert!(MonoThread::from_ptr(ptr::null_mut()).is_none());
    assert!(MonoType::from_ptr(ptr::null_mut()).is_none());
    assert!(MonoVTable::from_ptr(ptr::null_mut()).is_none());
}

#[test]
fn from_ptr_nonnull_returns_some_for_all_types() {
    let ptr = fake_ptr();
    assert!(MonoArray::from_ptr(ptr).is_some());
    assert!(MonoAssembly::from_ptr(ptr).is_some());
    assert!(MonoClass::from_ptr(ptr).is_some());
    assert!(MonoClassField::from_ptr(ptr).is_some());
    assert!(MonoDomain::from_ptr(ptr).is_some());
    assert!(MonoImage::from_ptr(ptr).is_some());
    assert!(MonoMethod::from_ptr(ptr).is_some());
    assert!(MonoObject::from_ptr(ptr).is_some());
    assert!(MonoString::from_ptr(ptr).is_some());
    assert!(MonoThread::from_ptr(ptr).is_some());
    assert!(MonoType::from_ptr(ptr).is_some());
    assert!(MonoVTable::from_ptr(ptr).is_some());
}

#[test]
fn as_ptr_round_trips_for_representative_types() {
    let ptr = fake_ptr();
    assert_eq!(MonoArray::from_ptr(ptr).unwrap().as_ptr(), ptr);
    assert_eq!(MonoClass::from_ptr(ptr).unwrap().as_ptr(), ptr);
    assert_eq!(MonoDomain::from_ptr(ptr).unwrap().as_ptr(), ptr);
    assert_eq!(MonoImage::from_ptr(ptr).unwrap().as_ptr(), ptr);
    assert_eq!(MonoMethod::from_ptr(ptr).unwrap().as_ptr(), ptr);
    assert_eq!(MonoObject::from_ptr(ptr).unwrap().as_ptr(), ptr);
}

#[test]
fn from_ptr_unchecked_matches_from_ptr() {
    let ptr = fake_ptr();
    let via_checked = MonoClass::from_ptr(ptr).unwrap().as_ptr();
    let via_unchecked = unsafe { MonoClass::from_ptr_unchecked(ptr) }.as_ptr();
    assert_eq!(via_checked, via_unchecked);
}

#[test]
fn handles_are_copy() {
    let ptr = fake_ptr();
    let a = MonoClass::from_ptr(ptr).unwrap();
    let b = a;
    // both remain usable because Copy semantics apply
    assert_eq!(a.as_ptr(), b.as_ptr());
}

#[test]
fn handles_implement_clone_trait() {
    // verify the Clone bound is satisfied by calling it through an explicit trait path
    fn cloned<T: Clone>(v: &T) -> T {
        v.clone()
    }
    let ptr = fake_ptr();
    let a = MonoImage::from_ptr(ptr).unwrap();
    let b = cloned(&a);
    assert_eq!(a.as_ptr(), b.as_ptr());
}

#[test]
fn handle_debug_output_is_non_empty() {
    let ptr = fake_ptr();
    let cls = MonoClass::from_ptr(ptr).unwrap();
    assert!(!format!("{cls:?}").is_empty());
}
