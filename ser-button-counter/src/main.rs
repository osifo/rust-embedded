
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::log::EspLogger;
use log::info;

static FLAG: AtomicBool = AtomicBool::new(false);

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();

    let mut press_count = 0_u32; //variable to track button press

     // this is a global variable i'm using to dertermine whether an interrupt has been triggered.


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
    unsafe { button.subscribe(gpio_int_callback).unwrap() }

    // enable the interupt
    button.enable_interrupt().unwrap();
    
    // main loop
    loop {
        let flag_val = FLAG.load(Ordering::Relaxed);

        if flag_val {
            FLAG.store(false, Ordering::Relaxed);
            button.enable_interrupt().unwrap();

            press_count =  press_count.wrapping_add(1); 
            info!("button press count is {}", press_count);
        }
    }
}

fn gpio_int_callback() {
    FLAG.store(true, Ordering::Relaxed);
}
