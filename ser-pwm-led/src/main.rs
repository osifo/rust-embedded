use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::ledc::{
    config::TimerConfig,
    LedcDriver,
    LedcTimerDriver,
    Resolution
};
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::sys::EspError;

fn main() -> Result<(), EspError> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    let timer_driver: LedcTimerDriver<'_, _> = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::default()
            .frequency(1000.Hz())
            .resolution(Resolution::Bits14)
    )?;

    let mut pwm_driver = LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        peripherals.pins.gpio7
    )?;

    setup_fading(&mut pwm_driver)
}

fn setup_fading(pwm: &mut LedcDriver) -> Result<(), EspError> {
    let start_time = std::time::Instant::now(); // gets the current time
    let fade_steps = 10;
    let mut duty_cycle: u128;

    // this value is based on the resolution of the timers used in the microcontroller for this example.
    // the ESP32S3 has 14-bit timers, as seen in the TimerConfig line above.
    let max_duty_amplitude = 16383; // 2^ nos of bits - 1
    let full_cycle_amplitude = max_duty_amplitude * 2;

    
    loop {
        let mut elapsed_time = start_time.elapsed().as_millis();
        let mapped_raw_step = (elapsed_time / fade_steps) % full_cycle_amplitude;
        if elapsed_time <= max_duty_amplitude {
            // this 'if' captures the first half of the cycle (the ON phase)
            duty_cycle = mapped_raw_step;
        } else if elapsed_time > max_duty_amplitude && elapsed_time <= full_cycle_amplitude  {
            // this captured the second half of the PWM, when the led is turnning off
            duty_cycle = full_cycle_amplitude - mapped_raw_step;
        } else {
            elapsed_time = 0_u128;
        }
    
        pwm.set_duty(duty_cycle as u32)?
    }
}