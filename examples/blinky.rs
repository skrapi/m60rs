#![no_main]
#![no_std]

use nrf52840_hal::gpio::Level;
use nrf52840_hal::prelude::{InputPin, OutputPin};

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = nrf52840_hal::pac::Peripherals::take().unwrap();
    let port0 = nrf52840_hal::gpio::p0::Parts::new(p.P0);
    let button = port0.p0_27.into_pullup_input();
    let mut red_led = port0.p0_30.into_push_pull_output(Level::Low);

    loop {
        if button.is_high().unwrap() {
            red_led.set_high().unwrap();
        } else {
            red_led.set_low().unwrap();
        }
    }
}
