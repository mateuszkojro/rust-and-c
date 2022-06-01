use std::ffi::CStr;
use rust_and_c::{ add, sub, foo, Vec2 };

fn main() {
    unsafe {
        println!("{}, {}, {:?}, {}, {:?}", add(2,2), sub(2,2), foo(), CStr::from_ptr(foo()).to_str().unwrap(), Vec2 {x: 1, y: 2} );
    }
}
