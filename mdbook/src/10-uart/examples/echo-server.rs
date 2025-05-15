#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::hal::uarte::{self, Baudrate, Parity};

use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // Main loop
    loop {
        // Attempt to read a byte from the serial port
        match serial.read() {
            Ok(byte) => {
                // Attempt to echo the byte back to the serial port
                if let Err(e) = serial.write(byte) {
                    rprintln!("Error writing to serial: {:?}", e);
                } else {
                    // Convert the byte to an ASCII character and print it
                    let ascii_char = char::from(byte);
                    rprintln!("Received and echoed: {}", ascii_char);
                }
            }
            Err(e) => rprintln!("Error reading from serial: {:?}", e),
        }
    }
}
