use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::timer::config::Config;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::timer::TimerDriver;
use esp_idf_svc::hal::gpio::PinDriver;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // setup gpio peripherals 
    let peripherals = Peripherals::take().expect("unable to instantiate peripherals");
    let mut button = PinDriver::input(peripherals.pins.gpio21).expect("could not initiate pin for button");
    let mut display_led = PinDriver::output(peripherals.pins.gpio14).expect("could not initiate pin for display led");
    let mut success_led = PinDriver::output(peripherals.pins.gpio12).expect("could not initiate pin for success led");
    let mut success_led = PinDriver::output(peripherals.pins.gpio7).expect("could not initiate pin for failure led");

    //setup timer peripherals
    let timer_config = Config::new().auto_reload(true);
    let timer = TimerDriver::new(peripherals.pins.timer00, &timer_config);

    timer.set_counter(0_u64).expect("could not initialize counter");

    
    //setup in
}
