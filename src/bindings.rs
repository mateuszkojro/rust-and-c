/* automatically generated by rust-bindgen 0.59.2 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Vec2() {
    assert_eq!(
        ::std::mem::size_of::<Vec2>(),
        8usize,
        concat!("Size of: ", stringify!(Vec2))
    );
    assert_eq!(
        ::std::mem::align_of::<Vec2>(),
        4usize,
        concat!("Alignment of ", stringify!(Vec2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Vec2>())).x as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Vec2), "::", stringify!(x))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Vec2>())).y as *const _ as usize },
        4usize,
        concat!("Offset of field: ", stringify!(Vec2), "::", stringify!(y))
    );
}
extern "C" {
    pub fn foo() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn add(a: ::std::os::raw::c_int, b: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sub(a: ::std::os::raw::c_int, b: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
