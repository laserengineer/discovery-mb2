#![no_main]
#![no_std]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use nrf52833_pac as _;
use nrf52833_pac::Peripherals;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = Peripherals::take().unwrap();
    rprintln!("Initialize Peripherals");
    p.P0.pin_cnf[21].write(|w| w.dir().output());
    rprintln!("Config Pin_21 as output");
    p.P0.pin_cnf[28].write(|w| w.dir().output());
    rprintln!("Config Pin_28 as output");

    let mut is_on: bool = false; // State variable to track if the LED is on or off.

    loop {
        p.P0.out.write(|w| w.pin21().bit(is_on));
        rprintln!("Write Pin_21 value :{}", is_on);
        for _ in 0..400_000 {
            nop();
        }
        is_on = !is_on;
    }
}
