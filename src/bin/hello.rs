#![no_main]
#![no_std]

use m60rs as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    m60rs::exit()
}
