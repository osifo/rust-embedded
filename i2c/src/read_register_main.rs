#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::hal::prelude::*;

#[cfg(feature = "v2")]
use microbit::{ 
  hal::twim, 
  pac::twim0::frequency::FREQUENCY_A,
};

const ACCELEROMETER_ADDR: u8 = 0b0011001;
const MAGNETOMETER_ADDR: u8 = 0b0011110;

//  location/address in memory where the register various sensors ID are located.
const ACCELEROMETER_ID_REG: u8 = 0x0f;
const MAGNETOMETER_ID_REG: u8 = 0x4f;

#[entry]
fn main () -> ! {
  rtt_init_print!();
  let board = microbit::Board::take().unwrap();

  #[cfg(feature = "v2")]
  let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

  let mut acc = [0];
  let mut mag = [0];

  i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc);
  i2c.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag);

  rprintln!("The accelerometer address is {}.", acc[0]);
  rprintln!("The magnetometer address is {}.", mag[0]);

  loop {}

}
