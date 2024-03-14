#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use core::fmt::Write;


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

  write!(serial, "Welcome to bare metal programming in rust\r\n").unwrap();
  nb::block!(serial.write(b'X')).unwrap();
  nb::block!(serial.flush()).unwrap();

  loop {
    let input_byte  = nb::block!(serial.read()).unwrap();

    write!(serial, "{}", input_byte as char);
    nb::block!(serial.flush()).unwrap();
  }
}
