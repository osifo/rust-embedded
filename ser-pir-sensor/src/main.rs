use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::log::EspLogger;
use std::sync::Mutex;

static PIR_SENSOR: Mutex<Option<PinDriver<Gpio16, Input>>> = Mutex::new(None);
static LED: Mutex<Option<PinDriver<Gpio5, Output>>> = Mutex::new(None);


// this application turns on the led once once motion is detected, and turns off in sync with the sensor PoSEdge change.
fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();

    // take the peripheral
    let peripherals = Peripherals::take().unwrap();

    let mut led_pin = PinDriver::output(peripherals.pins.gpio5).unwrap();
    let mut pir_pin = PinDriver::input(peripherals.pins.gpio16).unwrap();

    // set direction and interrupt type for input pin
    pir_pin.set_pull(Pull::Up).unwrap();

    //interupt type is set to AnyEdge, so it listens to both rising and falling edges.
    pir_pin.set_interrupt_type(InterruptType::AnyEdge).unwrap();

    // attach interrupt_handler to input pin
    unsafe { pir_pin.subscribe(pir_motion_handler).unwrap() }
 
    // enable the interrupt on the pin.
    pir_pin.enable_interrupt().unwrap();

    //populate the static Mutexes with values
    *PIR_SENSOR.lock().unwrap() = Some(pir_pin);
    *LED.lock().unwrap() = Some(led_pin);

    // empty loop so the device operates in low-power mode (sleeps when inactive).
    loop {}
}

fn pir_motion_handler() ->() {
    let mut pir_sensor_pin = PIR_SENSOR.lock().unwrap();
    let mut led_pin = LED.lock().unwrap();

    if let Some(pir_sensor) = pir_sensor_pin.as_mut() {
        if let Some(led) = led_pin.as_mut() {
            if pir_sensor.is_high() {
                led.set_level(Level::High).unwrap();
            } else {
                led.set_level(Level::Low).unwrap();
            }
        }
        
        pir_sensor.enable_interrupt().unwrap();
    }
}