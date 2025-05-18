#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]

fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut index: usize = 0;
    let coords: [(u8, u8); 16] = [
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (1, 4),
        (2, 4),
        (3, 4),
        (4, 4),
        (4, 3),
        (4, 2),
        (4, 1),
        (4, 0),
        (3, 0),
        (2, 0),
        (1, 0),
    ];

    loop {
        // Get the current coordinate from the coords array
        let (row, col) = coords[index];
        rprintln!("Current location is ({}, {})", row, col);

        // Generate the LED array with the current coordinate
        let led_array = generate_array_with_index(row as usize, col as usize);

        // Show the LED array for 1000ms
        display.show(&mut timer, led_array, 100);

        // Clear the display
        // display.clear();
        // timer.delay_ms(100_u32);

        // Move to the next coordinate
        index = (index + 1) % coords.len();
    }
}

fn generate_array_with_index(row: usize, col: usize) -> [[u8; 5]; 5] {
    // Ensure the indices are within bounds
    if row >= 5 || col >= 5 {
        panic!("Row and column indices must be within the range 0 to 4.");
    }

    // Initialize a 5x5 array with all zeros
    let mut array = [[0; 5]; 5];

    // Set the chosen element to 1
    array[row][col] = 1;

    // Return the array
    array
}
