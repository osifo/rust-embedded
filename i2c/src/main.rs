#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use lsm303agr::{AccelOutputDataRate, MagOutputDataRate, Measurement, Lsm303agr };
use heapless::Vec;
use core::str;
use core::fmt::Write;

#[cfg(feature = "v2")]
use microbit::{
  pac::twim0::frequency::FREQUENCY_A,
  board::Board,
  hal::{uarte, Timer, twim, prelude::* },
  hal::uarte::{Baudrate, Parity}
};

#[cfg(feature = "v2")]
mod serial_setup;

#[cfg(feature = "v2")]
use serial_setup::UartePort;


#[entry]
fn main() -> ! {
  rtt_init_print!();
  let board = Board::take().unwrap();

  let mut input_buffer: Vec<u8, 64> = Vec::new();
  let mut serial = {
    let serial  = uarte::Uarte::new(
      board.UARTE0,
      board.uart.into(),
      Parity::EXCLUDED,
      Baudrate::BAUD115200
    );

    UartePort::new(serial)
  };

  //initialize i2c chip
  let chip = {
    twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100)
  };

  let mut sensor = Lsm303agr::new_with_i2c(chip);
  sensor.init().unwrap();
  sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
  sensor.set_mag_odr(MagOutputDataRate::Hz50).unwrap();
  let mut sensor = sensor.into_mag_continuous().ok().unwrap();

  write!(serial, "\r\nEnter 'acc'/'mag' for accelerometer/magnetometer value:\r\n").unwrap();

  loop {
    let input_byte: u8 = nb::block!(serial.read()).unwrap();

    if input_buffer.push(input_byte).is_err() {
      write!(serial, "error: buffer is full!\r\n").unwrap();
      // break;
    }
    
    if input_byte == 13 { // if the return key is pressed
      write!(serial, "\r\n").unwrap();
      let prompt =  str::from_utf8(&input_buffer).unwrap();

      // gets the sensor reference
      if prompt.trim() == "acc"  {
        if sensor.accel_status().unwrap().xyz_new_data {
          let accel_data =  sensor.accel_data().unwrap();
          write!(serial, "{} values are - x: {}, y: {}, z: {}\r\n", prompt.trim(), accel_data.x, accel_data.y, accel_data.z).unwrap();
        }
      } else if prompt.trim() == "mag" {
        if sensor.mag_status().unwrap().xyz_new_data {
          let sensor_data =  sensor.mag_data().unwrap();
          write!(serial, "{} values are - x: {}, y: {}, z: {}\r\n", prompt.trim(), sensor_data.x, sensor_data.y, sensor_data.z).unwrap();
          // write!(serial, "{} - {:?} \n\r", prompt, sensor_data);
        }
      } else {
        write!(serial, "\r\nAn invalid prompt string - {} - was detected.\r\n", prompt).unwrap();
        // break;
      }

      input_buffer.clear();
    } else {
      write!(serial,"{}", input_byte as char);
    }
  }
}