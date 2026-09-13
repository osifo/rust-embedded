use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::ledc::{
    LedcDriver,
    LedcTimerDriver,
    config::TimerConfig,
    Resolution
};
use esp_idf_svc::sys::EspError;
use esp_idf_svc::hal::delay::FreeRtos;

fn main() -> Result<(), EspError> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take()?;
    let pwm_pin = peripherals.pins.gpio4;

    let timer_driver: LedcTimerDriver<'_,_> = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::default()
            .frequency(50.Hz())
            .resolution(Resolution::Bits10)
    )?;

    let mut pwm_driver = LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        pwm_pin
    )?;

    let pwm_max_duty = pwm_driver.get_max_duty() as f64;
    let min_duty = (0.05 * pwm_max_duty) as u32;
    let max_duty = (0.1 * pwm_max_duty) as u32;

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    setup_rotation(min_duty, max_duty, &mut pwm_driver)
}

fn setup_rotation(min_duty: u32, max_duty: u32, pwm: &mut LedcDriver<'_>) -> Result<(), EspError> {

    pwm.set_duty(min_duty)?;
    pwm.enable()?;

    loop {
        for duty in min_duty..max_duty {
            pwm.set_duty(duty)?;
            FreeRtos::delay_ms(20);
        }

        for duty in (min_duty..=max_duty).rev() {
            pwm.set_duty(duty)?;
            FreeRtos::delay_ms(20);
        }
    }
}
