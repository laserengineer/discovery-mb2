#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayMs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

// Constants for display size
const DISPLAY_ROWS: usize = 5;
const DISPLAY_COLS: usize = 5;

// Predefined sequence of LED coordinates
#[rustfmt::skip]
const PIXELS: [(usize, usize); 16] = [
    (0, 0), (0, 1), (0, 2), (0, 3), (0, 4),
    (1, 4), (2, 4), (3, 4), (4, 4), (4, 3),
    (4, 2), (4, 1), (4, 0), (3, 0), (2, 0), (1, 0),
];

#[entry]
fn main() -> ! {
    // Initialize RTT for debugging
    rtt_init_print!();
    rprintln!("Program started!");

    // Take ownership of the board
    let board = Board::take().expect("Failed to take ownership of the board.");
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    // Initialize the LED array dynamically
    let mut leds = [[0; DISPLAY_COLS]; DISPLAY_ROWS];

    // Track the last lit LED
    let mut last_led = (0, 0);

    // Main loop
    loop {
        for &current_led in PIXELS.iter() {
            // Log the current LED being lit
            rprintln!("Lighting up LED at ({}, {})", current_led.0, current_led.1);

            // Turn off the previous LED and light up the current LED
            leds[last_led.0][last_led.1] = 0;
            leds[current_led.0][current_led.1] = 1;

            // Display the updated LED array
            display.show(&mut timer, leds, 100);

            // Update the last lit LED
            last_led = current_led;
        }
    }
}
