use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::timer::config::Config;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::timer::TimerDriver;
use esp_idf_svc::hal::gpio::PinDriver;
use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};
// use esp_idf_svc::hal::delay::FreeRtos;

static IS_BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);
static HAS_DISPLAY_TIMEDOUT: AtomicBool = AtomicBool::new(false);

const MAX_DISPLAY_DURATION: i32 = 1500;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // setup gpio peripherals 
    let peripherals = Peripherals::take().expect("unable to instantiate peripherals");
    let mut button = PinDriver::input(peripherals.pins.gpio21).expect("could not initiate pin for button");
    let mut display_led = PinDriver::output(peripherals.pins.gpio14.downgrade_output()).expect("could not initiate pin for display led");
    let mut success_led = PinDriver::output(peripherals.pins.gpio12.downgrade_output()).expect("could not initiate pin for success led");
    let mut fail_led = PinDriver::output(peripherals.pins.gpio7.downgrade_output()).expect("could not initiate pin for failure led");
    
    let mut led_pins  = [display_led, success_led, fail_led];

    // connect an ISR to the button for button-press events
    button.set_pull(Pull::Up).expect("could not set pull type for button");
    button.set_interrupt_type(InterruptType::PosEdge).expect("could not set interrupt type");
    
    unsafe { button.subscribe(user_button_pressed).expect("could not connect the button to the ISR"); }
    button.enable_interrupt().expect("could not enable interrupt");
    
    //setup timer peripherals
    let timer_config: Config = Config::new().auto_reload(true);
    let mut response_timer = TimerDriver::new(peripherals.timer00, &timer_config).unwrap();
    let mut display_duration_timer = TimerDriver::new(peripherals.timer01, &timer_config).expect("could not initialize the display duration counter")

    let mut random = rand::rng();
    let turn_on_delay: i64 = random.random_range(1..=5);
    let on_duration: i32 = random.random_range(500..=MAX_DISPLAY_DURATION);

    response_timer.set_counter(0_u64).expect("could not initialize counter");
    response_timer.enable(true).expect("Could not enable timer");

    // setup the timer for measuring how long the led has been on for
    display_duration_timer.set_counter(0_u64);
    display_duration_timer.set_alarm(MAX_DISPLAY_DURATION as u64);
    unsafe { display_duration_timer.subscribe(handle_display_timeout).expect("unable to setup ISR for when the display resets"); }

    fn reset_display_leds(all_leds: [PinDriver<'_, AnyOutputPin, Output>; 3]) {
        for mut led in all_leds {
            led.set_level(Level::Low).expect("could not reset led display")
        }
    }

    fn calculate_response_delay(response_counter: TimerDriver) -> u64 {
        let press_delay = response_counter.counter().expect("could not get response counter value ");
        println!("user pressed the button {} after.", press_delay/1000);
        
        press_delay/1000
    }


    loop {
        if IS_BUTTON_PRESSED.load(Ordering::Relaxed) {
            if display_led.is_set_high() {
                calculate_response_delay(response_timer);
                display_led.set_level(Level::Low);
                success_led.set_level(Level::High);
                initialize_counters();
            }
        }

        if HAS_DISPLAY_TIMEDOUT.load(Ordering::Relaxed) {
            reset_display_leds(led_pins);
        }
    }
}

fn user_button_pressed() {
    IS_BUTTON_PRESSED.store(true, Ordering::Relaxed);
}

fn handle_display_timeout() {
    HAS_DISPLAY_TIMEDOUT.store(true, Ordering::Relaxed);
}
