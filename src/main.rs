#![no_std]
#![no_main]

mod base;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello, world!");
    println!("this is a simple OS kernel written in Rust.");

    panic!("failed to fail a failure");

    #[allow(clippy::empty_loop)]
    loop {}
}
