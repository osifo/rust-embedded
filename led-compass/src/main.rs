#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use libm::atan;
use lsm303agr::Measurement;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

mod calibration;
use crate::calibration::calc_calibration;
use crate::calibration::calibrated_measurement;
use microbit::{display::blocking::Display, hal::Timer, hal::prelude::*};

// #[cfg(feature = "v2")]
use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A};
use lsm303agr::{AccelOutputDataRate, Lsm303agr, MagOutputDataRate};
use libm::atan2f;
use core::f32::consts::PI;

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
        // let data =  Measurement {x: 409073, y: 439562, z: -697027};
        let x = data.x as f32;
        let y = data.y as f32;
        let z = data.z as f32;

        let theta = atan2f(x, y);
        let magnitude = x*x + y*y + z*z;

        let direction = if theta < -7. * PI / 8. {
            Direction::West
        } else if theta < -5. * PI / 8. {
            Direction::SouthWest
        } else if theta < -3. * PI / 8. {
            Direction::South
        } else if theta < -PI / 8. {
            Direction::SouthEast
        } else if theta < PI / 8. {
            Direction::East
        } else if theta < 3. * PI / 8. {
            Direction::NorthEast
        } else if theta < 5. * PI / 8. {
            Direction::North
        } else if theta < 7. * PI / 8. {
            Direction::NorthWest
        } else {
            Direction::West
        };

        display.show(&mut timer, direction_to_led(direction), 100);

        rprintln!("theta degrees ====== {}", theta * (180.0 / PI));
        rprintln!("magnitude ====== {}", magnitude);
        timer.delay_ms(1_000_u32)

    }
}
