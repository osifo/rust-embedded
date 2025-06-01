use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::adc::oneshot::*;
use esp_idf_svc::hal::adc::oneshot::config::{AdcChannelConfig, Calibration};
use esp_idf_svc::hal::adc::Resolution;
use esp_idf_svc::hal::adc::attenuation::DB_11;

const BASE: f64 = 0.0;
const VMAX: f64 = 1023.0;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let adc1 = AdcDriver::new(peripherals.adc1).unwrap();

    let led_a1  = PinDriver::output(peripherals.pins.gpio14.downgrade_output()).unwrap();
    let led_a2  = PinDriver::output(peripherals.pins.gpio13.downgrade_output()).unwrap();
    let led_a3  = PinDriver::output(peripherals.pins.gpio12.downgrade_output()).unwrap();
    let led_a4  = PinDriver::output(peripherals.pins.gpio11.downgrade_output()).unwrap();
    let led_a5  = PinDriver::output(peripherals.pins.gpio10.downgrade_output()).unwrap();
    let led_a6  = PinDriver::output(peripherals.pins.gpio9.downgrade_output()).unwrap();
    let led_a7  = PinDriver::output(peripherals.pins.gpio46.downgrade_output()).unwrap();
    let led_a8  = PinDriver::output(peripherals.pins.gpio3.downgrade_output()).unwrap();
    let led_a9  = PinDriver::output(peripherals.pins.gpio8.downgrade_output()).unwrap();
    let led_a10 = PinDriver::output(peripherals.pins.gpio18.downgrade_output()).unwrap();

    let mut led_bars: [PinDriver<'_, AnyOutputPin, Output>; 10] = [
        led_a1, led_a2, led_a3, led_a4, led_a5, led_a6, led_a7, led_a8, led_a9, led_a10
    ];
    
    let adc_config = AdcChannelConfig {
        calibration: Calibration::Curve,
        attenuation: DB_11,
        resolution: Resolution::Resolution12Bit
    };

    let mut adc_channel = AdcChannelDriver::new(
        &adc1, 
        peripherals.pins.gpio1,
        &adc_config
    ).unwrap();

    fn get_ranged_value(sampled_reading: f64) -> f64 {
        let min_output_value: f64 = 0.0;
        let max_output_value: f64 = 10.0;

        (min_output_value + (max_output_value - min_output_value) * ((sampled_reading - BASE) / (VMAX - BASE))) / 5.0
    }

    fn illuminate_bar_graph(
        ledbars: &mut [PinDriver<'_, AnyOutputPin, Output>; 10], 
        value: u16
    ) {
        for (index, led) in ledbars.iter_mut().enumerate() {
            if index as u16 <= value {
                led.set_level(Level::High).unwrap();
            } else {
                led.set_level(Level::Low).unwrap();
            }
        }
    }

    loop {
        let sample_reading = adc_channel.read_raw().unwrap();
        let ranged_reading =  get_ranged_value(sample_reading as f64).ceil() as u16;

        illuminate_bar_graph(&mut led_bars, ranged_reading);

        FreeRtos::delay_ms(200);
    }

}
