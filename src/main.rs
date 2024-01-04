#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::base::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod base;

fn main() {
    println!("hello, world!");
    println!("this is a simple OS kernel written in Rust.");
}
