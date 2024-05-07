/*
 * @Description: Description of this file
 * @Version: 2.0
 * @Author: 夏明
 * @Date: 2024-05-04 19:45:25
 * @LastEditors: 夏明
 * @LastEditTime: 2024-05-07 17:41:31
 */
#![no_std]
#![no_main]
mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    loop {}
}
