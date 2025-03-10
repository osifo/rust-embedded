use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Step 1 - Take the device peripherals
    let device_peri = Peripherals::take().unwrap();

    // Step 2 - Cofingure Pin direction (set as output pin)
    let mut led_pin = PinDriver::output(device_peri.pins.gpio1).unwrap();


    loop {
        // Step 3 - configure pin drive
        log::info!("setting signal to high");
        led_pin.set_high();
        FreeRtos::delay_ms(1000_u32);
        
        log::info!("setting signal to low");
        led_pin.set_low();
        FreeRtos::delay_ms(1000_u32);
    }
}
