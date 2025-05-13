use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::adc::oneshot::*;
use esp_idf_svc::hal::adc::oneshot::config::{AdcChannelConfig, Calibration};
use esp_idf_svc::hal::adc::Resolution;
use esp_idf_svc::hal::adc::attenuation::DB_11;

const BASE:f64 = 3950.0;
const VMAX: f64 =  4095.0;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let adc1 = AdcDriver::new(peripherals.adc1).unwrap();

    let mut led_a1  = PinDriver::output(peripherals.pins.gpio14.downgrade_output());
    let mut led_a2  = PinDriver::output(peripherals.pins.gpio13.downgrade_output());
    let mut led_a3  = PinDriver::output(peripherals.pins.gpio12.downgrade_output());
    let mut led_a4  = PinDriver::output(peripherals.pins.gpio11.downgrade_output());
    let mut led_a5  = PinDriver::output(peripherals.pins.gpio10.downgrade_output());
    let mut led_a6  = PinDriver::output(peripherals.pins.gpio9.downgrade_output());
    let mut led_a7  = PinDriver::output(peripherals.pins.gpio46.downgrade_output());
    let mut led_a8  = PinDriver::output(peripherals.pins.gpio3.downgrade_output());
    let mut led_a9  = PinDriver::output(peripherals.pins.gpio8.downgrade_output());
    let mut led_a10 = PinDriver::output(peripherals.pins.gpio18.downgrade_output());

    let led_pins: [PinDriver<'_, AnyOutputPin, Output>] = [
        led_a1, led_a2, led_a3, led_a4, led_a5, led_a6, led_a7, led_a8, led_a9, led_a10
    ];
    
    let adc_config = AdcChannelConfig {
        calibration: Calibration::Curve,
        attenuation: DB_11,
        resolution: Resolution::Resolution12Bit
    }

    let mut adc_channel = AdcChannelDriver::new(
        &adc1, 
        peripherals.pins.gpio48,
        &adc_config
    ).unwrap();

    fn get_ranged_value(sampled_reading: f64) -> u16 {
        const MIN_OUTPUT_VALUE = 1;
        const MAX_OUTPUT_VALUE = 10;

        (sample_reading - BASE) * (MAX_OUTPUT_VALUE - MIN_OUTPUT_VALUE) / (VMAX - BASE) + MAX_OUTPUT_VALUE
    }

    fn illuminate_bar_graph(value: u32) {
        for (index, led) in led_pins.iter_mut().enumerate() {
            if index <= value {
                led.set_level(Level::High);
            } else {
                led.set_level(Level::Low);
            }
        }
    }

    loop {
        let sample_reading = adc_channel::read_raw().unwrap();
        let ranged_reading = get_ranged_value();

        illuminate_bar_graph(ranged_reading);

        FreeRtos::delay_ms(200);
    }

}
