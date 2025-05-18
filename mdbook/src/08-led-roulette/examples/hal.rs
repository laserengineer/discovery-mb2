#![no_main]
#![no_std]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::digital::{OutputPin, PinState};
use hal::{gpio::Level, pac};
use nrf52833_hal as hal;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = pac::Peripherals::take().unwrap();
    rprintln!("Initialize Peripherals");
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    rprintln!("Initialize Peripherals Port");
    let _col1 = port0.p0_28.into_push_pull_output(Level::Low);
    rprintln!("Initialize Peripherals Port 0 Pin 28");
    let mut row1 = port0.p0_21.into_push_pull_output(Level::Low);
    rprintln!("Initialize Peripherals Port 0 Pin 21");
    let mut is_on: bool = false; // State variable to track if the LED is on or off.

    loop {
        let _ = row1.set_state(PinState::from(is_on));
        rprintln!("Set row1 state: {}", is_on);
        for _ in 0..400_000 {
            nop();
        }
        is_on = !is_on;
    }
}
