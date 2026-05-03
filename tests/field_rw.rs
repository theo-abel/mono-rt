// These tests exercise read_field and write_field against plain stack-allocated
// byte arrays, so no Mono runtime is required.

use std::ffi::c_void;

#[test]
fn read_field_u8_at_offset() {
    let mut data = [0u8; 8];
    data[3] = 0xAB;
    let val: u8 = unsafe { mono_rt::read_field(data.as_mut_ptr().cast::<c_void>(), 3) };
    assert_eq!(val, 0xAB);
}

#[test]
fn read_field_u32_at_offset() {
    let mut data = [0u8; 8];
    data[4..8].copy_from_slice(&0x1234_5678_u32.to_ne_bytes());
    let val: u32 = unsafe { mono_rt::read_field(data.as_mut_ptr().cast::<c_void>(), 4) };
    assert_eq!(val, 0x1234_5678);
}

#[test]
fn write_field_stores_u32() {
    let mut data = [0u8; 8];
    let ptr = data.as_mut_ptr().cast::<c_void>();
    unsafe { mono_rt::write_field(ptr, 0, 0xDEAD_BEEF_u32) };
    let val: u32 = unsafe { mono_rt::read_field(ptr, 0) };
    assert_eq!(val, 0xDEAD_BEEF);
}

#[test]
fn write_then_read_i32_negative() {
    let mut data = [0u8; 8];
    let ptr = data.as_mut_ptr().cast::<c_void>();
    unsafe { mono_rt::write_field(ptr, 0, -1_i32) };
    let val: i32 = unsafe { mono_rt::read_field(ptr, 0) };
    assert_eq!(val, -1);
}

#[test]
fn write_then_read_f32() {
    let mut data = [0u8; 8];
    let ptr = data.as_mut_ptr().cast::<c_void>();
    unsafe { mono_rt::write_field(ptr, 0, std::f32::consts::PI) };
    let val: f32 = unsafe { mono_rt::read_field(ptr, 0) };
    assert!((val - std::f32::consts::PI).abs() < f32::EPSILON);
}

#[test]
fn write_then_read_f64() {
    let mut data = [0u8; 16];
    let ptr = data.as_mut_ptr().cast::<c_void>();
    unsafe { mono_rt::write_field(ptr, 0, std::f64::consts::E) };
    let val: f64 = unsafe { mono_rt::read_field(ptr, 0) };
    assert!((val - std::f64::consts::E).abs() < f64::EPSILON);
}

#[test]
fn write_then_read_bool() {
    let mut data = [0u8; 4];
    let ptr = data.as_mut_ptr().cast::<c_void>();
    unsafe { mono_rt::write_field(ptr, 0, true) };
    let val: bool = unsafe { mono_rt::read_field(ptr, 0) };
    assert!(val);
}

#[test]
fn write_at_unaligned_offset_u16() {
    // offset 1 is not naturally aligned for u16; read_unaligned must handle this
    let mut data = [0u8; 8];
    let ptr = data.as_mut_ptr().cast::<c_void>();
    unsafe { mono_rt::write_field(ptr, 1, 0xABCD_u16) };
    let val: u16 = unsafe { mono_rt::read_field(ptr, 1) };
    assert_eq!(val, 0xABCD);
}

#[test]
fn write_does_not_clobber_neighboring_bytes() {
    let mut data = [0u8; 12];
    let ptr = data.as_mut_ptr().cast::<c_void>();
    unsafe { mono_rt::write_field(ptr, 4, 0x1234_u32) };
    // bytes before offset 4 must still be zero
    let before: u32 = unsafe { mono_rt::read_field(ptr, 0) };
    assert_eq!(before, 0);
    // bytes after offset 8 must still be zero
    let after: u32 = unsafe { mono_rt::read_field(ptr, 8) };
    assert_eq!(after, 0);
}

#[test]
fn write_field_overwrites_previous_value() {
    let mut data = [0xFFu8; 8];
    let ptr = data.as_mut_ptr().cast::<c_void>();
    unsafe { mono_rt::write_field(ptr, 0, 0_u32) };
    let val: u32 = unsafe { mono_rt::read_field(ptr, 0) };
    assert_eq!(val, 0);
}
