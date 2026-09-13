#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::hal::prelude::*;
use lsm303agr::{AccelOutputDataRate, Lsm303agr, AccelMode};

#[cfg(feature = "v2")]
use microbit::{
  hal::twim,
  pac::twim0::frequency::FREQUENCY_A,
  board::Board,
  hal::Timer
};


#[entry]
fn main() -> ! {
  rtt_init_print!();
  let board = microbit::Board::take().unwrap();
  let mut timer = Timer::new(board.TIMER0);

  let i2c_chip = { 
    twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100)
  };

  let mut sensor = Lsm303agr::new_with_i2c(i2c_chip);
  sensor.init().unwrap();
  sensor
      .set_accel_odr(AccelOutputDataRate::Hz50)
      .unwrap();

  loop {
      if sensor.accel_status().unwrap().xyz_new_data {
          let data = sensor.accel_data().unwrap();
          rprintln!(
              "Acceleration: x {} y {} z {}",
              data.x,
              data.y,
              data.z
          );
      }

      timer.delay_ms(1_000_u32)
  } 
}