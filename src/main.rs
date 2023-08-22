#![no_std]
#![no_main]

static HELLO: &[u8] = b"Hello World!";

use core::panic::PanicInfo;

/// This function is a panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    let vga_buffer = 0xb8000 as *mut u8; // pointer to the start of the vga buffer

    for(i, &byte) in HELLO.iter().enumerate() { 
        unsafe{
            *vga_buffer.offset(i as isize * 2) = byte; // write the byte to the vga buffer
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // give the byte color
        }
    }
    loop {}
}

