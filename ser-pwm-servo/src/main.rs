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
            .frequency(1000.Hz())
            .resolution(Resolution::Bits14)
    )?;

    let mut pwm_driver = LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        pwm_pin
    )?;

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    setup_rotation(&mut pwm_driver)
}

fn setup_rotation(pwm: &mut LedcDriver) -> Result<(), EspError> {
    let start_time = std::time::Instant::now();
    let fade_frequency_hz = 1.0; // this defines the number of cycles per sec
    let cycle_time_ms = 1000.0 / fade_frequency_hz;
    
    let max_duty_amplitude: f64 = 16383.0;
    let full_cycle_amplitude: f64 = max_duty_amplitude * 2.0;

    let fade_steps = cycle_time_ms / max_duty_amplitude;

    loop {
        let elapsed_time = start_time.elapsed().as_millis() as f64;
        
        // this mapping ensurs that when the step gets to the max allowed (amplitude), it resets to 0
        let mapped_raw_step = (elapsed_time / fade_steps) % full_cycle_amplitude;

        let duty_cycle = if mapped_raw_step <= max_duty_amplitude {
            mapped_raw_step
        } else {
            full_cycle_amplitude - mapped_raw_step
        };

        pwm.set_duty(duty_cycle as u32);

        FreeRtos::delay_ms(0);
    }
}

