#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::base::tests::test_runner)]

mod base;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello, world!");
    println!("this is a simple OS kernel written in Rust.");

    #[allow(clippy::empty_loop)]
    loop {}
}
