use esp_idf_svc::hal::adc::attenuation::DB_11;
use esp_idf_svc::hal::adc::Resolution;
use esp_idf_svc::hal::adc::oneshot::config::{AdcChannelConfig, Calibration};
use esp_idf_svc::hal::adc::oneshot::*;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::peripherals::Peripherals;


fn main() {
    const VMAX: f64 =  4095.0;

    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let adc1 = AdcDriver::new(peripherals.adc1).unwrap();

    //set up the ADC channel configuration
    let channel_config = AdcChannelConfig {
        attenuation: DB_11,
        calibration: Calibration::Curve,
        resolution: Resolution::Resolution12Bit
    };

    let mut adc_channel = AdcChannelDriver::new(
        &adc1,
        peripherals.pins.gpio10,
        &channel_config
    ).unwrap();

    loop {
        let sample_reading: u16 = adc_channel.read().unwrap();
        let raw_reading: u16 =  adc_channel.read_raw().unwrap();
        let input_voltage: <<<f64 as Div<i32>>::Output as BitXor<i32>>::Output as Mul<u16>>::Output = (VMAX / 2^8) * sampled_value;

        println!("Digital Reading: {}, Voltage Reading: {}", sample_reading, raw_reading);

        FreeRtos::delay_ms(500);
    }
}
