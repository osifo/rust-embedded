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
use esp_idf_svc::hal::adc::ADC1;

fn main() -> Result<(), EspError> {

    let peripherals = Peripherals::take()?;

    let led_pin = peripherals.pins.gpio4;
    let potentiometer_iopin = peripherals.pins.gpio14;

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

    let channel_config = AdcChannelConfig {
        attenuation: DB_11,
        calibration: Calibration::Curve,
        resolution: AdcResolution::Resolution12Bit
    };

    let mut pot_adc = AdcChannelDriver::new(
        AdcDriver::new(peripherals.adc2)?,
        peripherals.pins.gpio14,
        &channel_config
    )?;

    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    loop {
        // Read potentiometer ADC value
        let pot_reading = get_pot_reading(&mut pot_adc)?;
        
        
        // Map ADC reading (0-4095) to PWM duty cycle (0-16383 for 14-bit resolution)
        let max_duty = (1u32 << 14) - 1; // 16383
        let duty = ((raw as u64 * max_duty as u64) / 4095u64) as u32;
        
        // Set PWM duty cycle
        pwm_driver.set_duty(duty)?;
        
        // Small delay to avoid overwhelming the ADC/PWM
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

}

fn get_pot_reading(pot_adc: &mut AdcChannelDriver<AdcDriver<ADC1>, Gpio14>) -> Result<u64, EspError> {
    let adc_max = 4095u64; // 12-bit ADC max value
    let pwm_max = ((1u64 << 14) - 1) as u64; // Max duty cycle for 14-bit resolution
    
   
    let pot_reading = pot_adc.read_raw()?;
    // Map ADC reading to PWM duty cycle
    let duty_cycle = (pot_reading as u64 * pwm_max) / adc_max;
    
    Ok(duty_cycle)
}

fn setup_pwm_from_potentiometer(pwm: &mut LedcDriver, pot_pin: &PinDriver<'_, Gpio14, Input>) -> Result<(), EspError> {
    // Read the potentiometer value (this is a placeholder; actual ADC reading code needed)
    let pot_value = 0; // Replace with actual ADC read

    // Map potentiomete r value to PWM duty cycle
    let duty_cycle = pot_value as u32; // Adjust mapping as necessary

    pwm.set_duty(duty_cycle)
}