use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    //step 1 - take the peripherals
    let device_peri = Peripherals::take().unwrap();
    let blink_delay = 500_u32;

    // Step 2 - set the pin direction for the pins i want to use
    let a10_pin = PinDriver::output(device_peri.pins.gpio1.downgrade_output()).unwrap();
    let a9_pin  = PinDriver::output(device_peri.pins.gpio10.downgrade_output()).unwrap();
    let a8_pin  = PinDriver::output(device_peri.pins.gpio19.downgrade_output()).unwrap();
    let a7_pin  = PinDriver::output(device_peri.pins.gpio18.downgrade_output()).unwrap();
    let a6_pin  = PinDriver::output(device_peri.pins.gpio4.downgrade_output()).unwrap();
    let a5_pin  = PinDriver::output(device_peri.pins.gpio5.downgrade_output()).unwrap();
    let a4_pin  = PinDriver::output(device_peri.pins.gpio6.downgrade_output()).unwrap();
    let a3_pin  = PinDriver::output(device_peri.pins.gpio7.downgrade_output()).unwrap();
    let a2_pin  = PinDriver::output(device_peri.pins.gpio8.downgrade_output()).unwrap();
    let a1_pin  = PinDriver::output(device_peri.pins.gpio9.downgrade_output()).unwrap();

    let mut pin_list  = [a10_pin, a9_pin, a8_pin, a7_pin, a6_pin, a5_pin, a4_pin, a3_pin, a2_pin, a1_pin];
    let mut button_pin = PinDriver::input(device_peri.pins.gpio3).unwrap();

    // Step 3 - Set pin pull (for input pin)
    button_pin.set_pull(Pull::Up).unwrap();

    loop {
        for mut pin in &mut pin_list {
            pin.set_high().unwrap(); // turn on the led
            let delay_val = button_pressed(&button_pin, &blink_delay); // get on delay duration 
            FreeRtos::delay_ms(delay_val); // keep the led on
    
            pin.set_low().unwrap(); // turn off the led
            FreeRtos::delay_ms(blink_delay); // keep the led off 
        }
    }
}

fn button_pressed(button: &PinDriver<'_, Gpio3, Input>, delay: &u32) -> u32 {
    if button.is_low() {
        println!("button pressed");
        
        // upon each press, the delay is reduced by 50. it is reset to 200 once it's <= 50.
        if delay <= &100_u32 {
            return 1000_u32;
        } else {
            return delay - 100_u32;
        }
    } else {
        return *delay;
    }
}
