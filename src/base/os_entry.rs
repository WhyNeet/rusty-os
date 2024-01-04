#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(not(test))]
    crate::main();

    #[cfg(test)]
    crate::test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}
