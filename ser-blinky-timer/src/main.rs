use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::timer::TimerDriver;
use esp_idf_svc::hal::timer::config::Config;
use std::sync::atomic::{AtomicBool, Ordering};

const LED_TOGGLE_DURATION: u64 = 1000000_u64;
static IS_LED_ON: AtomicBool = AtomicBool::new(false);

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // 1. define the GPIO peripherals (pins) to be used - as well as how they're used.
    // NOTE: defining the ground pin isn't necessary.
    let peripherals = Peripherals::take().unwrap();
    let mut led_pin = PinDriver::output(peripherals.pins.gpio1).unwrap();

    // 2. setup the timer peripheral
    let timer_config = Config::new().auto_reload(true);
    let mut timer = TimerDriver::new(peripherals.timer00, &timer_config).unwrap();

    // setup the timer's configurations //
    timer.set_counter(0_u64).unwrap();
    
    // when this value is reached, the timer's reset (see auto_reload config) and interrupts are triggered
    timer.set_alarm(LED_TOGGLE_DURATION).unwrap();
    timer.enable_alarm(true).unwrap();
    timer.enable(true).unwrap();

    // // 3. connect the timer's interrupt to a handler logic(isr)
    unsafe { timer.subscribe(toggle_led_state).unwrap(); }
    timer.enable_interrupt().unwrap();

    // the logic keeps running indefinitely
    loop {
        if IS_LED_ON.load(Ordering::Relaxed) {
            led_pin.set_level(Level::High).expect("Could not update led state");
        } else {
            led_pin.set_level(Level::Low).expect("Could not update led state");
        }
    }
}


fn toggle_led_state() {        
    if IS_LED_ON.load(Ordering::Relaxed) {
        IS_LED_ON.store(false, Ordering::Relaxed);
    } else {
        IS_LED_ON.store(true, Ordering::Relaxed);
    }
}