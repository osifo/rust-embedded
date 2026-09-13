// #![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

mod calibration;

#[cfg(feature = "v2")]
mod serial_setup;

use crate::calibration::calc_calibration;
use crate::calibration::calibrated_measurement;
use microbit::{display::blocking::Display, hal::Timer, hal::prelude::*};


#[cfg(feature = "v2")]
use microbit::{
    hal::{uarte, twim, prelude::*},
    pac::twim0::frequency::FREQUENCY_A,
};
use lsm303agr::{AccelOutputDataRate, Lsm303agr, MagOutputDataRate, AccelScale};

#[entry]
fn main() -> ! {
    let mut max_acc: f32 = 0.00;
    let mut acc_threshold: f32 = 100.00;
    const MEASURE_INTERVAL: u32 = 3_000_u32;
    let mut isMeasuring = false;

    rtt_init_print!();

    let board = microbit::Board::take().unwrap();
    let chip = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut timer = Timer::new(board.TIMER0);

    let mut sensor = Lsm303agr::new_with_i2c(chip);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();

    let _ = sensor.set_accel_scale(AccelScale::G2).unwrap();
    

    loop {
        let accel_data = sensor.accel_data().unwrap();
        let new_acc_value = accel_data.x as f32;
        
        if isMeasuring {
            if new_acc_value > max_acc {
                max_acc = new_acc_value;
                
                // write!(serial, "There is a new highest acceleration value --- {}", new_data.x).unwrap();
                rprintln!("There is a new highest acceleration value --- {}", new_acc_value);
            }
            max_acc = 0.00;
            isMeasuring = false;
        } else {
            if new_acc_value > acc_threshold {
                isMeasuring = true;
                max_acc = new_acc_value;

            }
        }

        timer.delay_ms(1_000_u32);
    }
}