#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(writing_an_os_in_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use writing_an_os_in_rust::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    writing_an_os_in_rust::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello Yuchanns{}", "!");

    writing_an_os_in_rust::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    writing_an_os_in_rust::hlt_loop();
}
