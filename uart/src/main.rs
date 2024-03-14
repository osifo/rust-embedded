#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use core::fmt::Write;
use heapless::Vec;


#[cfg(feature = "v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;

#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
  rtt_init_print!();
  let board = microbit::Board::take().unwrap();

  #[cfg(feature = "v2")]
  let mut serial = {
    let serial = uarte::Uarte::new(
      board.UARTE0,
      board.uart.into(),
      Parity::EXCLUDED,
      Baudrate::BAUD115200,
    );

    UartePort::new(serial) // there should be no semicolon here because of the implicit return.
  };

  let mut buffer: Vec<char, 32> = Vec::new();

  loop {
    let input_byte: u8  = nb::block!(serial.read()).unwrap();

    if input_byte == 13 {
      buffer.reverse(); 
      write!(serial, "\n\n\rThe reversed is: ");

      for xter in buffer.iter() {
        write!(serial, "{}",  &xter);
      }

      write!(serial, "\n\n\r");
      buffer.clear();
    }

    buffer.push(input_byte as char);
      write!(serial, "{}", input_byte as char);
    
  }
}
