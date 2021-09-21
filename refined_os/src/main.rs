#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

/// A function that should never return is marked as a diverging function
/// by returning the "never" type `!`.

/// This function is called on panic.
#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

static HELLO: &[u8] = b"Welcome to RefinedOS!";
/// Overwriting the entry point as we don't have access
/// to the Rust runtime and `crt0`.
/// Disable name mangling to tell the linker the name of the entry point function.
/// Use `C` calling convention
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to RefinedOS!");
    
    loop {}
}