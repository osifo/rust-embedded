use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::adc::{Resolution, ADC1};
use esp_idf_svc::hal::adc::oneshot::*;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::adc::oneshot::config::{AdcChannelConfig, Calibration};
use esp_idf_svc::hal::adc::attenuation::DB_11;
use esp_idf_svc::hal::peripherals::Peripherals;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};
use libm::log;

static CURRENT_TEMP_VALUE: AtomicU32 = AtomicU32::new(0);
static LED: Mutex<Option<PinDriver<Gpio13, Output>>> = Mutex::new(None);

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
    *LED.lock().unwrap() = Some(PinDriver::output(peripherals.pins.gpio13).unwrap());
    
    // let thermistor_pin = PinDriver::input(peripherals.pins.gpio9).unwrap();

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


    // thermistor_pin.set_pull(Pull::Up).unwrap();
    // thermistor_pin.set_interrupt_type(InterruptType::AnyEdge);
    // unsafe { thermistor_pin.subscribe(handle_temperature_change).unwrap() }
    // thermistor_pin.enable_interrupt().unwrap();

    fn update_blink_frequency(temp_value: u32) -> () { 
        let mut led_pin = LED.lock().unwrap();
        let blink_interval = (100 - temp_value) * 20;

        if let Some(led) = led_pin.as_mut() {
            loop {
                led.set_level(Level::High).unwrap();
                FreeRtos::delay_ms(blink_interval);

                led.set_level(Level::Low).unwrap();
                FreeRtos::delay_ms(blink_interval);
            }
        }
    }
    
    // returns a value only if the temperature has changed since last sampling
    fn check_for_temperature_change<'a>(channel: &mut AdcChannelDriver<'a, Gpio4, &AdcDriver<'a, ADC1>>) -> Option<u32>  {
        let sample_reading: u16 = channel.read_raw().unwrap();

        //calculate temperature
        let sampled_value = (1.0 / (log(1.0/(VMAX / sample_reading as f64 - 1.0)) / B + 1.0 / 298.15) - 273.15).ceil() as u32;

        let curr_temperature = CURRENT_TEMP_VALUE.load(Ordering::Relaxed);
        
        if sampled_value == curr_temperature {
            Some(curr_temperature)
        } else {
            CURRENT_TEMP_VALUE.store(sampled_value, Ordering::Relaxed);
            Some(sampled_value)
        }
    }

    loop {
        if let Some(temperature_value) = check_for_temperature_change(&mut adc_channel) {
            update_blink_frequency(temperature_value);
        } else {
            update_blink_frequency(20);
        }

        FreeRtos::delay_ms(200);
    };
}
