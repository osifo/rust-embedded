#![deny(unsafe_code)]
#![no_std]
#![no_main]


use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{Timer, prelude::*}
};

#[entry]

fn main() -> ! { 
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds: [[u8; 5]; 5] = [
        [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]
    ];
    let active_cells: [(usize, usize); 16] = [(0,0), (0,1), (0,2), (0,3), (0,4), (1,4), (2,4), (3,4), (4,4), (4,3), (4,2), (4,1), (4,0), (3, 0), (2,0), (1, 0)];

    let mut previous_led: (usize, usize) = (0,0);
    
    loop {
        for current_led in active_cells.iter() {
            leds[previous_led.0][previous_led.1] = 0;
            leds[current_led.0][current_led.1] = 1;

            display.show(&mut timer, leds, 100);

            previous_led = *current_led;
        }
    
        timer.delay_ms(1_000_u32)
    }
}
