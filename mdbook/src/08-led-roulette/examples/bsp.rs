#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{board::Board, hal::timer::Timer};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    let _ = board.display_pins.col1.set_low();
    let mut row1 = board.display_pins.row1;
    let mut row2 = board.display_pins.row2;

    loop {
        rprintln!("Set LED LOW");
        let _ = row1.set_low();
        let _ = row2.set_low();
        timer.delay_ms(1_000);
        rprintln!("Set LED HIGH");
        let _ = row1.set_high();
        let _ = row2.set_high();
        timer.delay_ms(1_000);
    }
}
