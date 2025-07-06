use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::adc::Resolution;
use esp_idf_svc::hal::adc::oneshot::*;
use esp_idf_svc::hal::adc::oneshot::config::{AdcChannelConfig, Calibration};
use esp_idf_svc::hal::adc::attenuation::DB_11;
use esp_idf_svc::hal::peripherals::Peripherals;
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

    let channel_config = AdcChannelConfig {
        calibration: Calibration::Curve,
        attenuation: DB_11,
        resolution: Resolution::Resolution12Bit
    };

    let mut adc_channel = AdcChannelDriver::new(
        &adc1, 
        peripherals.pins.gpio4,
        &channel_config
    ).unwrap();

    loop {
        let sample_reading: u16 = adc_channel.read_raw().unwrap();

        //calculate temperature
        let temp = 1.0 / (log(1.0/(VMAX / sample_reading as f64 - 1.0)) / B + 1.0 / 298.15) - 273.15;

        println!("Temperature is {}", temp);
        FreeRtos::delay_ms(500);
    };
}
