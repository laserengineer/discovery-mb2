#![no_main]
#![no_std]

use core::ptr::write_volatile;
use cortex_m::asm::nop;
use nrf52833_pac as _;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello World from Micro Bit v2!");
    const GPIO0_PINCNF21_ROW1_ADDR: *mut u32 = 0x5000_0754 as *mut u32; // Address for ROW1 configuration.
    const GPIO0_PINCNF28_COL1_ADDR: *mut u32 = 0x5000_0770 as *mut u32; // Address for COL1 configuration.
    const DIR_OUTPUT_POS: u32 = 0; // Bit position for output direction.
    const PINCNF_DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS; // Configuration value to set the pin as output.
                                                       // Config ROW1 AND COL1 as output
    unsafe {
        rprintln!(
            "write_volatile: {:?}, {:?}",
            GPIO0_PINCNF21_ROW1_ADDR,
            PINCNF_DRIVE_LED
        );
        write_volatile(GPIO0_PINCNF21_ROW1_ADDR, PINCNF_DRIVE_LED); // Set ROW1 as output
        rprintln!(
            "write_volatile: {:?}, {:?}",
            GPIO0_PINCNF28_COL1_ADDR,
            PINCNF_DRIVE_LED
        );
        write_volatile(GPIO0_PINCNF28_COL1_ADDR, PINCNF_DRIVE_LED); // Set COL1 as output.
    }

    const GPIO0_OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32; // Address for GPIO output.
    const GPIO0_OUT_ROW1_POS: u32 = 21; // Bit position for ROW1 in the output register.
    let mut is_on: bool = false; // State variable to track if the LED is on or off.

    loop {
        rprintln!("LED Value: {}", is_on);
        unsafe {
            rprintln!(
                "write_volatile: {:?}, {:?}",
                GPIO0_OUT_ADDR,
                (is_on as u32) << GPIO0_OUT_ROW1_POS
            );
            write_volatile(GPIO0_OUT_ADDR, (is_on as u32) << GPIO0_OUT_ROW1_POS);
        }
        for _ in 0..400_000 {
            nop();
        }
        is_on = !is_on;
    }
}
