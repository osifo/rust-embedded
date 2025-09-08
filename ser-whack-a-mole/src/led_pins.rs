use std::sync::atomic::{AtomicBool, Ordering};
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::gpio::PinDriver;

use crate::timers::Timers;

pub struct LedPins {
    pub display: PinDriver<'static, Gpio14, Output>,
    pub success: PinDriver<'static, Gpio12, Output>,
    pub fail: PinDriver<'static, Gpio7, Output>
}

impl LedPins {
    pub fn new(display_pin: Gpio14, success_pin: Gpio12, failure_pin: Gpio7) -> Self {
        let display = PinDriver::output(display_pin).expect("could not initiate pin for display led");
        let success = PinDriver::output(success_pin).expect("could not initiate pin for success led");
        let fail = PinDriver::output(failure_pin).expect("could not initiate pin for failure led");
    
        LedPins {
            display,
            success,
            fail
        }
    }

    pub fn reset_all_feedback(&mut self) {
        self.success.set_level(Level::Low).expect("could not turn off success led.");
        self.fail.set_level(Level::Low).expect("could not turn off fail led.");
    }

    pub fn display_success_feedback(&mut self, response_speed: u64, timers: &mut Timers, feedback_check: AtomicBool) {
        self.display.set_level(Level::Low).expect("could not turn off display led."); // turns off display led
        self.success.set_level(Level::High).expect("could not turn on success led.");
        
        feedback_check.store(true, Ordering::Relaxed);
        timers.track_feedback_timer();

        println!("You responded in {} ms", response_speed);
    }

    pub fn display_failure_feedback(&mut self, timers: &mut Timers, feedback_check: AtomicBool) {
        self.display.set_level(Level::Low).expect("could not turn off display led."); // turns off display led
        self.fail.set_level(Level::High).expect("could not turn on failure led.");
        
        feedback_check.store(true, Ordering::Relaxed);
        timers.track_feedback_timer();
    }
}
