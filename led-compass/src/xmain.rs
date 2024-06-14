#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use lsm303agr::Measurement;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

mod calibration;
use crate::calibration::calc_calibration;
use crate::calibration::calibrated_measurement;
use microbit::{display::blocking::Display, hal::Timer};

#[cfg(feature = "v2")]
use microbit::{
  hal::twim, 
  pac::twim0::frequency::FREQUENCY_A
};
use lsm303agr::{AccelOutputDataRate, Lsm303agr, MagOutputDataRate};

mod led;
use crate::led::Direction;
use crate::led::direction_to_led;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v2")]
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz10).unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    let calibration = calc_calibration(&mut sensor, &mut display, &mut timer);
    // rprintln!("Calibration: {:?}", calibration);
    rprintln!("Calibration done, entering busy loop");
    loop {
        while !sensor.mag_status().unwrap().xyz_new_data {}
        let mut mag_data = sensor.mag_data().unwrap();
        let data = calibrated_measurement(mag_data, &calibration);
        // data =  Measurement {x: 409073, y: 439562, z: -697027};
        
        // let direction: Direction = match (data.x > 0, data.y > 0) {
        //   (true, true) => Direction::NorthWest,
        //   (false, false) => Direction::SouthEast,
        //   (true, false) => Direction::SouthWest,
        //   (false, true) => Direction::NorthEast,
        // };

        let mut direction = Direction::North; 

        direction = if data.x > 0 && data.y > 0 { Direction::NorthWest } 
        else if data.x == 0 && data.y < 0 { Direction::South }
        else if data.x < 0 && data.y == 0 { Direction::East }
        else if data.x < 0 && data.y < 0 { Direction::SouthEast }
        else if data.x > 0 && data.y == 0 { Direction::West }
        else if data.x > 0 && data.y < 0 { Direction::SouthWest }
        else if data.x == 0 && data.y > 0 { Direction::North }
        else { Direction::NorthEast };
        // else if data.x < 0 && data.y > 0 { Direction::NorthEast }

        display.show(&mut timer, direction_to_led(direction), 100);
        // timer.delay_ms(1_000_u32)
    }
}
