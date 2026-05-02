// Each file under tests/ compiles as its own binary (fresh process), so BINDINGS
// is guaranteed to be empty throughout this suite — no init() call ever succeeds here.

use std::ffi::c_void;

use mono_rt::{
    MonoArray, MonoAssembly, MonoClass, MonoClassField, MonoDomain, MonoError, MonoImage,
    MonoMethod, MonoObject, MonoString, MonoType,
};

fn fake_ptr() -> *mut c_void {
    std::ptr::dangling_mut::<c_void>()
}

// MonoDomain

#[test]
fn domain_root_requires_init() {
    assert!(matches!(MonoDomain::root(), Err(MonoError::Uninitialized)));
}

#[test]
fn domain_open_assembly_requires_init() {
    let domain = unsafe { MonoDomain::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(
        domain.open_assembly("Assembly-CSharp.dll"),
        Err(MonoError::Uninitialized)
    ));
}

// MonoImage

#[test]
fn image_find_requires_init() {
    assert!(matches!(
        MonoImage::find("Assembly-CSharp"),
        Err(MonoError::Uninitialized)
    ));
}

#[test]
fn image_class_from_name_requires_init() {
    let image = unsafe { MonoImage::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(
        image.class_from_name("", "Player"),
        Err(MonoError::Uninitialized)
    ));
}

// MonoAssembly

#[test]
fn assembly_image_requires_init() {
    let asm = unsafe { MonoAssembly::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(asm.image(), Err(MonoError::Uninitialized)));
}

// MonoClass

#[test]
fn class_field_lookup_requires_init() {
    let cls = unsafe { MonoClass::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(cls.field("health"), Err(MonoError::Uninitialized)));
}

#[test]
fn class_method_lookup_requires_init() {
    let cls = unsafe { MonoClass::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(
        cls.method("Update", None),
        Err(MonoError::Uninitialized)
    ));
}

#[test]
fn class_method_with_arity_requires_init() {
    let cls = unsafe { MonoClass::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(
        cls.method("TakeDamage", Some(1)),
        Err(MonoError::Uninitialized)
    ));
}

#[test]
fn class_mono_type_requires_init() {
    let cls = unsafe { MonoClass::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(cls.mono_type(), Err(MonoError::Uninitialized)));
}

#[test]
fn class_vtable_requires_init() {
    let cls = unsafe { MonoClass::from_ptr_unchecked(fake_ptr()) };
    let domain = unsafe { MonoDomain::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(cls.vtable(domain), Err(MonoError::Uninitialized)));
}

#[test]
fn class_new_object_requires_init() {
    let cls = unsafe { MonoClass::from_ptr_unchecked(fake_ptr()) };
    let domain = unsafe { MonoDomain::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(
        cls.new_object(domain),
        Err(MonoError::Uninitialized)
    ));
}

#[test]
fn class_fields_requires_init() {
    let cls = unsafe { MonoClass::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(cls.fields(), Err(MonoError::Uninitialized)));
}

#[test]
fn class_methods_requires_init() {
    let cls = unsafe { MonoClass::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(cls.methods(), Err(MonoError::Uninitialized)));
}

// MonoClassField

#[test]
fn field_offset_requires_init() {
    let field = unsafe { MonoClassField::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(field.offset(), Err(MonoError::Uninitialized)));
}

#[test]
fn field_name_requires_init() {
    let field = unsafe { MonoClassField::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(field.name(), Err(MonoError::Uninitialized)));
}

#[test]
fn field_mono_type_requires_init() {
    let field = unsafe { MonoClassField::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(field.mono_type(), Err(MonoError::Uninitialized)));
}

// MonoMethod

#[test]
fn method_name_requires_init() {
    let method = unsafe { MonoMethod::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(method.name(), Err(MonoError::Uninitialized)));
}

#[test]
fn method_invoke_requires_init() {
    let method = unsafe { MonoMethod::from_ptr_unchecked(fake_ptr()) };
    let result = unsafe { method.invoke(std::ptr::null_mut(), std::ptr::null_mut()) };
    assert!(matches!(result, Err(MonoError::Uninitialized)));
}

#[test]
fn method_invoke_with_requires_init() {
    let method = unsafe { MonoMethod::from_ptr_unchecked(fake_ptr()) };
    let result = unsafe { method.invoke_with(std::ptr::null_mut(), &[]) };
    assert!(matches!(result, Err(MonoError::Uninitialized)));
}

// MonoObject

#[test]
fn object_unbox_requires_init() {
    let obj = unsafe { MonoObject::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(obj.unbox(), Err(MonoError::Uninitialized)));
}

// MonoArray

#[test]
fn array_len_requires_init() {
    let arr = unsafe { MonoArray::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(arr.len(), Err(MonoError::Uninitialized)));
}

#[test]
fn array_is_empty_requires_init() {
    let arr = unsafe { MonoArray::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(arr.is_empty(), Err(MonoError::Uninitialized)));
}

#[test]
fn array_addr_requires_init() {
    let arr = unsafe { MonoArray::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(arr.addr(4, 0), Err(MonoError::Uninitialized)));
}

// MonoString

#[test]
fn string_new_with_valid_text_requires_init() {
    let domain = unsafe { MonoDomain::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(
        MonoString::new(domain, "hello"),
        Err(MonoError::Uninitialized)
    ));
}

#[test]
fn string_to_string_lossy_requires_init() {
    let s = unsafe { MonoString::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(s.to_string_lossy(), Err(MonoError::Uninitialized)));
}

// MonoType

#[test]
fn mono_type_kind_requires_init() {
    let t = unsafe { MonoType::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(t.kind(), Err(MonoError::Uninitialized)));
}

#[test]
fn mono_type_object_requires_init() {
    let t = unsafe { MonoType::from_ptr_unchecked(fake_ptr()) };
    let domain = unsafe { MonoDomain::from_ptr_unchecked(fake_ptr()) };
    assert!(matches!(t.object(domain), Err(MonoError::Uninitialized)));
}

// MonoThreadGuard

#[test]
fn thread_guard_attach_requires_init() {
    let result = unsafe { mono_rt::MonoThreadGuard::attach() };
    assert!(matches!(result, Err(MonoError::Uninitialized)));
}
