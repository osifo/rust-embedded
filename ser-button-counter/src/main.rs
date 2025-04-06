
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::hal::delay::FreeRtos;
use std::sync::Mutex;
// use esp_idf_svc::hal::mutex::Mutex;
use log::info;

static PRESS_COUNT: AtomicU32 = AtomicU32::new(0);
static BUTTON_PIN: Mutex<Option<PinDriver<Gpio0, Input>>> = Mutex::new(None);

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();

    // 1. tke peripheral
    let dp = Peripherals::take().unwrap();

    // 2. configure pin direction
    let mut button = PinDriver::input(dp.pins.gpio0).unwrap();

    // 3. set pull type (for input pin)
    // this connects the pin to the supply voltage when it's in `active high` state
    button.set_pull(Pull::Up).unwrap();

    /* 
        because it's an output pin, we can set the interrupt type. 
        here's it's set to trigger on the rising(+ve) edge of the input signal
    */
    button.set_interrupt_type(InterruptType::PosEdge).unwrap();

    // this connects the button interrupt to the ISR
    unsafe { button.subscribe(button_press_callback).unwrap() }

    // enable the interupt
    button.enable_interrupt().unwrap();

    // this sets the button_pin within a mutex lock
    *BUTTON_PIN.lock().unwrap() = Some(button);
    
    // main loop
    loop {
        let press_count =  PRESS_COUNT.load(Ordering::Relaxed); 
        FreeRtos::delay_ms(1000_u32);
        info!("button press count is {}", press_count);
    }
}

fn button_press_callback() {
    let press_count = PRESS_COUNT.fetch_add(1, Ordering::Relaxed);

    let mut button_pin = BUTTON_PIN.lock().unwrap();

    // this borrows the button pin val as mutable, without moving (taking ownership)
    if let Some(mut button) = button_pin.as_mut() {
      button.enable_interrupt().unwrap();
    }

}
