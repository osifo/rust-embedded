use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::adc::Resolution;
use esp_idf_svc::hal::adc::oneshot::*;
use esp_idf_svc::hal::gpio::*;
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
    
    let a1_pin = PinDriver::output(peripherals.pins.gpio14.downgrade_output()).unwrap();
    let a2_pin = PinDriver::output(peripherals.pins.gpio13.downgrade_output()).unwrap();
    let a3_pin = PinDriver::output(peripherals.pins.gpio12.downgrade_output()).unwrap();
    let a4_pin = PinDriver::output(peripherals.pins.gpio11.downgrade_output()).unwrap();
    let a5_pin = PinDriver::output(peripherals.pins.gpio10.downgrade_output()).unwrap();
    let a6_pin = PinDriver::output(peripherals.pins.gpio9.downgrade_output()).unwrap();
    let a7_pin = PinDriver::output(peripherals.pins.gpio46.downgrade_output()).unwrap();
    let a8_pin = PinDriver::output(peripherals.pins.gpio3.downgrade_output()).unwrap();
    let a9_pin = PinDriver::output(peripherals.pins.gpio8.downgrade_output()).unwrap();
    let a10_pin = PinDriver::output(peripherals.pins.gpio18.downgrade_output()).unwrap();

    let mut pin_list: [PinDriver<'_, AnyOutputPin, Output>; 10] = [a1_pin, a2_pin, a3_pin, a4_pin, a5_pin, a6_pin, a7_pin, a8_pin, a9_pin, a10_pin];

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

    fn update_led_lighting(ledbars: &mut [PinDriver<'_, AnyOutputPin, Output>; 10], temperature_score: u32) {
        for (index, bar) in ledbars.iter_mut().enumerate() {
            if (index as u32) <= temperature_score {
                bar.set_level(Level::High).unwrap();
                FreeRtos::delay_ms(100)
            } else {
                bar.set_level(Level::Low).unwrap();
            }
        }
    }

    loop {
        let sample_reading: u16 = adc_channel.read_raw().unwrap();

        //calculate temperature
        let temp = 1.0 / (log(1.0/(VMAX / sample_reading as f64 - 1.0)) / B + 1.0 / 298.15) - 273.15;
        
        let scaled_temparature = ((temp / 80.0) * 10.0).ceil() as u32;

        update_led_lighting(&mut pin_list, scaled_temparature);
        FreeRtos::delay_ms(100);
    };
}
