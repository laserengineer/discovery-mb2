#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use heapless::Vec;
use panic_rtt_target as _;
use rtt_target::{rprint, rprintln, rtt_init_print};

use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

// Import the serial setup module
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    // Initialize RTT for debugging
    rtt_init_print!();
    rprintln!("Echo Program started!");
    let board = microbit::Board::take().unwrap();

    // Set up the serial port
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear(); // Clear the buffer at the start of each loop
        rprintln!("Waiting for user input...");

        // Receive user input (terminated by ENTER)
        loop {
            match serial.read() {
                Ok(byte) => {
                    // Check for ENTER (newline character '\n')
                    if byte == b'\n' || byte == b'\r' {
                        break; // End of user input
                    }
                    let input_char = char::from(byte);
                    rprintln!("Serial input received: {}", input_char);

                    // Attempt to push the byte into the buffer
                    if buffer.push(byte).is_err() {
                        // Buffer overflow: send an error message
                        let _ = serial.write(b'!');
                        rprintln!("Error: Buffer overflow. Input too long.");
                        break;
                    }
                }
                Err(e) => {
                    // Handle read error
                    rprintln!("Error reading from serial: {:?}", e);
                }
            }
        }
        rprintln!("Input loop break");

        // Reverse the string in the buffer
        let reversed: Vec<u8, 32> = buffer.iter().rev().cloned().collect();
        serial.flush().unwrap();
        // Send back the reversed string
        for &byte in reversed.iter() {
            let ascii_char = char::from(byte);
            write!(serial, "{}", ascii_char).unwrap(); // Use write! macro
        }

        // Send a newline character to indicate the end of the response
        let _ = serial.write(b'\r');
        rprintln!("Serial port write finished");
    }
}
