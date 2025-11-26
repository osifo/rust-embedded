use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::delay::Ets;

use std::fmt::Error;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let mut led_pin = PinDriver::output(peripherals.pins.gpio12).expect("could not set output pin");

    loop {
        emulate_pwm(&mut led_pin, 100, 1000, 10000).unwrap();
    }
}

fn emulate_pwm(pin: &mut PinDriver<'_, Gpio12, Output>, duty_level: i32, frequency_hz: i32, duration_ms: i32) -> Result<(), Error> {
    let period = 1_000_000 / frequency_hz;
    let fade_steps = 200;
    let half_duration = duration_ms / 2; // total duration is low->high and high-> low
    let duration_per_step_ms = half_duration / fade_steps;
    let cycles_per_step: i32 = (duration_per_step_ms * frequency_hz) / 1000;


    let start_time = std::time::Instant::now();

    while start_time.elapsed().as_millis() < duration_ms as u128 {
      // fade upwards
      for curr_duty_cycle in 0..=duty_level {
        if start_time.elapsed().as_millis() >= half_duration as u128{
            break;
        }

        log::info!("current cycle ======= {}", curr_duty_cycle);

        let on_time_us = (period * curr_duty_cycle) / 100;
        let off_time_us = period - on_time_us;

        for _ in 0..cycles_per_step {
            if on_time_us > 0 {
                pin.set_high().unwrap();
                Ets::delay_us(on_time_us as u32);
            }

            if off_time_us > 0 {
                pin.set_low().unwrap();
                Ets::delay_us(off_time_us as u32);
            }
        }
      }

      // fade down
      for curr_duty_cycle in (0..=duty_level).rev() {
        // let elapsed_time = start_time.elapsed().as_millis();
        if start_time.elapsed().as_millis() >= duration_ms as u128 {
            break;
        }

        let on_time_us = (period * curr_duty_cycle) / 100;
        let off_time_us = period - on_time_us;

        for _ in 0..cycles_per_step {
            if on_time_us > 0 {
                pin.set_high().unwrap();
                Ets::delay_us(on_time_us as u32);
            }

            if off_time_us > 0 {
                pin.set_low().unwrap();
                Ets::delay_us(off_time_us as u32);
            }
        }
      }
    }

    Ok(())
}
