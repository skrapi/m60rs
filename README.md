# M60.rs 

An attempt to learn Embedded Rust using the M60 keyboard from Makerdairy.

## Project Setup
To start I followed along with the instructions from [James Munns](https://github.com/jamesmunns/m60-keyboard).

Key points to watch out for:
1. 


## Blinky
Blinky is an example which turns on the red led if the reset button is pressed.

### Building blinky for release
```sh
cargo build --example blinky --release
```

### Flashing Blinky
```sh
/flash target/thumbv7em-none-eabihf/release/examples/blinky
```

## Resources
1. [Hardware set up](https://wiki.makerdiary.com/m60/developer_guide/hardware/)
1. [Python code](https://github.com/makerdiary/python-keyboard)
1. [Keyberon-F4](https://github.com/TeXitoi/keyberon-f4/blob/master/src/main.rs)
1. [nrf52840 Datasheet](https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.1.pdf)
1. [Example of flashing an embedded Rust application](https://beta7.io/posts/embedded-rust-from-zero-to-blinky.html)

