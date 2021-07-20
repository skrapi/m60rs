#![no_main]
#![no_std]

use nrf52840_hal::{
    gpio::Level, 
    prelude::{InputPin, OutputPin},
    Delay,
    
};


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
    let mut green_led = port0.p0_29.into_push_pull_output(Level::Low);
    let mut blue_led = port0.p0_31.into_push_pull_output(Level::Low);

    let delay = Delay::new();
    loop {
        red_led.set_high().unwrap();
        green_led.set_high().unwrap();
        blue_led.set_high().unwrap();
        red_led.set_low().unwrap();
        green_led.set_low().unwrap();
        blue_led.set_low().unwrap();
        delay.(1000);    
    }
}
