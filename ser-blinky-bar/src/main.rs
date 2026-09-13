use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::atomic::Ordering;

static IS_BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);
static ON_DELAY: AtomicU32 = AtomicU32::new(2000);

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    //step 1 - take the peripherals
    let device_peri = Peripherals::take().unwrap();
    let off_delay = 100_u32;

    // Step 2 - set the pin direction for the pins i want to use
    // PinDriver here is form the gpio crate imported above
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
    button_pin.set_interrupt_type(InterruptType::PosEdge).unwrap();
    
    // connects the button's interrupt to the defined ISR
    unsafe { button_pin.subscribe(button_pressed_callback).unwrap() }
    
    button_pin.enable_interrupt().unwrap();

    loop {
        for mut pin in &mut pin_list {
            pin.set_level(Level::High).unwrap(); // turn on the led
            // let delay_val = button_pressed(&button_pin, &blink_delay); // get on delay duration 
            FreeRtos::delay_ms(ON_DELAY.load(Ordering::Relaxed)); // keep the led on
    
            pin.set_level(Level::Low).unwrap(); // turn off the led
            FreeRtos::delay_ms(off_delay); // keep the led off

            handle_button_pressed(&mut button_pin);
        }
    }
}

fn handle_button_pressed(button: &mut PinDriver<'_, Gpio3, Input>) -> () {

    if IS_BUTTON_PRESSED.load(Ordering::Relaxed) { // checks if the button is pressed.

        let delay_value = ON_DELAY.load(Ordering::Relaxed);
        if delay_value <= 200 {
            ON_DELAY.store(2000, Ordering::Relaxed); // resets
        } else {
            ON_DELAY.store(delay_value.wrapping_sub(200), Ordering::Relaxed);
        }

        // resets the button-pressed state
        IS_BUTTON_PRESSED.store(false, Ordering::Relaxed);
        button.enable_interrupt().unwrap();
    }
}

fn button_pressed_callback() {
    IS_BUTTON_PRESSED.store(true, Ordering::Relaxed);
}
