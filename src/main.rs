use rust_and_c::add;

fn main() {
    unsafe {
        println!("Hello this is your clib {}", add(2,2));
    }
}
