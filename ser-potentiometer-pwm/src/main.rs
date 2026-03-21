use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals; 
use esp_idf_svc::hal::ledc::{
    LedcDriver,
    LedcTimerDriver, 
    Resolution
};
use esp_idf_svc::hal::ledc::config::TimerConfig;
use esp_idf_svc::sys::EspError;
use esp_idf_svc::hal::prelude::*;

use esp_idf_svc::hal::adc::attenuation::DB_11;
use esp_idf_svc::hal::adc::Resolution as AdcResolution;
use esp_idf_svc::hal::adc::oneshot::config::{AdcChannelConfig, Calibration};
use esp_idf_svc::hal::adc::oneshot::*;

fn main() -> Result<(), EspError> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let start_duty: u32 = 0;
    let peripherals = Peripherals::take()?;

    let led_pin = peripherals.pins.gpio4;
    let pot_pin = peripherals.pins.gpio10;

    let ledc_timer = LedcTimerDriver::new(
        peripherals.ledc.timer0, 
        &TimerConfig::default()
            .frequency(1000.Hz())
            .resolution(Resolution::Bits14)
    )?;

    let mut pwm_driver: LedcDriver<'_> = LedcDriver::new(
        peripherals.ledc.channel0, 
        ledc_timer,
        led_pin
    )?;

    let adc_channel_config = AdcChannelConfig {
        attenuation: DB_11,
        calibration: Calibration::Curve,
        resolution: AdcResolution::Resolution12Bit
    };

    let adc_driver = AdcDriver::new(peripherals.adc1)?;

    let mut pot_adc = AdcChannelDriver::new(
        &adc_driver,
        pot_pin,
        &adc_channel_config
    )?;

    
    loop {
        // Read potentiometer ADC value
        let adc_input = pot_adc.read_raw()?;
        log::info!("Initial ADC reading -------- {}", adc_input);

        let current_duty = adc_to_pwm_cycle(adc_input)?;
        log::info!("current duty -------- {}", current_duty);

        for duty in start_duty..current_duty {
            pwm_driver.set_duty(duty)?;
        }
        
        for reverse_duty in current_duty..start_duty {
            pwm_driver.set_duty(reverse_duty)?;
        }

        // std::thread::sleep(std::time::Duration::from_millis(500));
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}


fn adc_to_pwm_cycle(adc_input: u16) -> Result<u32, EspError> {
    let adc_max = 4095u64; // 12-bit ADC max value
    let pwm_max = ((1u64 << 14) - 1) as u64; // Max duty cycle for 14-bit resolution
    
    // Map ADC reading to PWM duty cycle
    let mapped_duty_cycle = ((adc_input as u64 * pwm_max) / adc_max) as u32;
    
    Ok(mapped_duty_cycle)
}
