#![no_main]
#![no_std]
#![feature(asm)]
use core::panic::PanicInfo;
use cortex_m_rt::entry;
use f3::hal::stm32f30x::{self};
use gpio::*;

#[entry]
fn main() -> ! {
    PortC::init();
    PortE::init();
    let mut button = PortC::pin_in::<6>();
    let mut led1 = PortE::pin_out::<9>();
    let mut led2 = PortE::pin_out::<13>();
    button.set_pull_down();
    loop {
        if button.read() == 1 {
            led1.set();
            led2.reset();
        } else {
            led1.reset();
            led2.set();
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
