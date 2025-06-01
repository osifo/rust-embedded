use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::adc::Resolution;
use esp_idf_svc::hal::adc::oneshot::*;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::adc::oneshot::config::{AdcChannelConfig, Calibration};
use esp_idf_svc::hal::adc::attenuation::DB_11;
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() {
    const BASE:f64 = 20.0;
    const VMAX: f64 = 2000.0;

    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let adc1 = AdcDriver::new(peripherals.adc1).unwrap();
    
    let led_a1 = PinDriver::output(peripherals.pins.gpio37.downgrade_output()).unwrap();
    let led_a2 = PinDriver::output(peripherals.pins.gpio36.downgrade_output()).unwrap();
    let led_a3 = PinDriver::output(peripherals.pins.gpio35.downgrade_output()).unwrap();
    let led_a4 = PinDriver::output(peripherals.pins.gpio0.downgrade_output()).unwrap();
    let led_a5 = PinDriver::output(peripherals.pins.gpio45.downgrade_output()).unwrap();
    let led_a6 = PinDriver::output(peripherals.pins.gpio48.downgrade_output()).unwrap();
    let led_a7 = PinDriver::output(peripherals.pins.gpio47.downgrade_output()).unwrap();
    let led_a8 = PinDriver::output(peripherals.pins.gpio21.downgrade_output()).unwrap();
    let led_a9 = PinDriver::output(peripherals.pins.gpio20.downgrade_output()).unwrap();
    let led_a10 = PinDriver::output(peripherals.pins.gpio19.downgrade_output()).unwrap();

    let mut ledbar: [PinDriver<'_, AnyOutputPin, Output>; 10] = [
        led_a1, 
        led_a2, 
        led_a3,
        led_a4,
        led_a5,
        led_a6,
        led_a7,
        led_a8,
        led_a9,
        led_a10
    ];

    let channel_config = AdcChannelConfig {
        calibration: Calibration::Curve,
        attenuation: DB_11,
        resolution: Resolution::Resolution12Bit
    };

    let mut adc_channel = AdcChannelDriver::new(
        &adc1, 
        peripherals.pins.gpio1, // this is pin to which the ADC output is connected
        &channel_config
    ).unwrap();

    fn get_ranged_value(sampled_reading: f64) -> u32 {
        let min_output_value: f64 = 0.0;
        let max_output_value: f64 = 9.0;

        if sampled_reading <= 20.0 {
            return 0;
        } else if sampled_reading > 2000.0 {
            return 9;
        }

        let scaled_value = (min_output_value + max_output_value - min_output_value) * ((sampled_reading - BASE) / (VMAX - BASE));
        
        scaled_value.ceil() as u32
    }

    fn light_leds(
        ledbar: &mut[PinDriver<'_, AnyOutputPin, Output>; 10], 
        light_value: u32
    ) {
        for (index, led) in ledbar.iter_mut().enumerate() {
            if (index as u32) <= light_value - 1 {
                led.set_level(Level::High).unwrap();
            } else {
                led.set_level(Level::Low).unwrap();
            }
        }
    }

    loop {
        let sample_reading: u16 = adc_channel.read_raw().unwrap();
        let scaled_reading = get_ranged_value(sample_reading.into());

        light_leds(&mut ledbar, scaled_reading);

        FreeRtos::delay_ms(100);
    };
}
