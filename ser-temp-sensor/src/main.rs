use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::adc::Resolution;
use esp_idf_svc::hal::adc::oneshot::*;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::adc::oneshot::config::{AdcChannelConfig, Calibration};
use esp_idf_svc::hal::adc::attenuation::DB_11;
use esp_idf_svc::hal::peripherals::Peripherals;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};
use libm::log;

fn main() {
    const B:f64 = 3950.0;
    const VMAX: f64 =  4095.0;

    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    
    let adc1 = AdcDriver::new(peripherals.adc1).unwrap();
    let mut led_pin = PinDriver::output(peripherals.pins.gpio13).unwrap();
    
    let channel_config = AdcChannelConfig {
        calibration: Calibration::Curve,
        attenuation: DB_11,
        resolution: Resolution::Resolution12Bit
    };

    let mut adc_channel = AdcChannelDriver::new(
        &adc1, 
        peripherals.pins.gpio9,
        &channel_config
    ).unwrap();

    fn update_blink_frequency(led: &mut PinDriver<'_, Gpio13, Output>, temp_value: u32) -> () { 
        let blink_interval = (105 - (temp_value + 24))* 30;

        led.set_level(Level::High).unwrap();
        FreeRtos::delay_ms(blink_interval);

        led.set_level(Level::Low).unwrap();
        FreeRtos::delay_ms(blink_interval);
    }
    
    // returns a value only if the temperature has changed since last sampling
    fn check_for_temperature_change(sample_reading: f64) -> u32 {

        //calculate temperature
        let sampled_value = (1.0 / (log(1.0/(VMAX / sample_reading - 1.0)) / B + 1.0 / 298.15) - 273.15).ceil() as u32;

        sampled_value
    }
    
    loop {
        let sample_reading: u16 = adc_channel.read_raw().unwrap();
        let temperature_value = check_for_temperature_change(sample_reading as f64);

        update_blink_frequency(&mut led_pin, temperature_value);
    }
}
