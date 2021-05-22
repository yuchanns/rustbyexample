#![no_std]
#![no_main]

use core::panic::PanicInfo;
use writing_an_os_in_rust::{exit_qemu, init, serial_print, serial_println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    double_fault_panic();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn double_fault_panic() {
    serial_print!("should_panic::double_fault_panic...\t");
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
