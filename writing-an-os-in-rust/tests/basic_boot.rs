#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(writing_an_os_in_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use writing_an_os_in_rust::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    writing_an_os_in_rust::test_panic_handler(_info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
