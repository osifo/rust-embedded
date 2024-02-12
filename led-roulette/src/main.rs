#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! { 
    rtt_init_print!();

    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut col1 = board.display_pins.col1;
    let mut col2 = board.display_pins.col2;
    let mut row1 = board.display_pins.row1;
    let mut row2 = board.display_pins.row2;

    loop {
        row1.set_high().unwrap();
        col1.set_high().unwrap();
        col2.set_low().unwrap();
        row2.set_low().unwrap();
        rprintln!("dark, light");

        timer.delay_ms(1000u16);

        row2.set_high().unwrap();
        col1.set_low().unwrap();

        row1.set_low().unwrap();
        col2.set_high().unwrap();
        rprintln!("light, dark");

        timer.delay_ms(1000u16);
    }
}
