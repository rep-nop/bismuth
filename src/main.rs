#![no_main]
#![no_std]

extern crate panic_itm;

use core::ptr::{read_volatile, write_volatile};
use cortex_m::asm::nop;
use cortex_m_rt::entry;

unsafe fn enable_led() {
    const PORT_B_BASE: usize = 0x4002_0400;
    const ENABLE_BIT_7: u32 = 0x01 << 14;

    let val = read_volatile(PORT_B_BASE as *mut u32) | ENABLE_BIT_7;

    write_volatile(PORT_B_BASE as *mut u32, val);

    const PORT_B_CLK_ENABLE: usize = 0x4000_0030;
    const ENABLE_PORT_B_CLK: u32 = 0x10;

    let val = read_volatile(PORT_B_CLK_ENABLE as *mut u32) | ENABLE_PORT_B_CLK;

    write_volatile(PORT_B_CLK_ENABLE as *mut u32, val);
}

unsafe fn toggle_led() {
    const PORT_B_OUT_DR: usize = 0x4002_0414;
    const LED_HIGH: u32 = 0x10;

    let val = read_volatile(PORT_B_OUT_DR as *mut u32) | LED_HIGH;

    write_volatile(PORT_B_OUT_DR as *mut u32, val);
}

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    unsafe {
        enable_led();
    }

    loop {
        for i in 0..100_000_000 {
            nop();
        }

        unsafe {
            toggle_led();
        }
    }
}
