#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// A function that should never return is marked as a diverging function
/// by returning the "never" type `!`.

/// This function is called on panic.
#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Welcome to RefinedOS!";
/// Overwriting the entry point as we don't have access
/// to the Rust runtime and `crt0`.
/// Disable name mangling to tell the linker the name of the entry point function.
/// Use `C` calling convention
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 11+(i as u8 % 5);
        }
    }

    loop {}
}