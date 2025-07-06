use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::timer::TimerDriver;
use esp_idf_svc::hal::timer::config::Config;
use std::sync::atomic::{AtomicBool, Ordering};
use std::ops::Add;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    static TIMER_FLAG: AtomicBool = AtomicBool::new(false);

    struct Time {
        seconds: u32,
        minutes: u32,
        hours: u32
    }

    let mut tracker = Time {
        seconds: 0_u32,
        minutes: 0_u32,
        hours: 0_u32,
    };

    // setup device-specific code
    let config = Config::new().auto_reload(true);
    let peripherals = Peripherals::take().unwrap();
    let mut timer = TimerDriver::new(peripherals.timer00, &config).unwrap();

    timer.set_counter(0_u64).unwrap();
    timer.set_alarm(timer.tick_hz()).unwrap();

    unsafe { timer.subscribe(trigger_time_tracker).unwrap() }

    timer.enable_interrupt().unwrap();
    timer.enable_alarm(true).unwrap();
    timer.enable(true).unwrap();

    //setup ISR
    fn trigger_time_tracker() {
        // set the flag to true every time the counter alarm is triggered.
        TIMER_FLAG.store(true, Ordering::Relaxed);
    }

    loop {
        if TIMER_FLAG.load(Ordering::Relaxed) {
            TIMER_FLAG.store(false, Ordering::Relaxed);

            tracker.seconds = tracker.seconds.wrapping_add(1);

            if tracker.seconds > 59 {
                tracker.minutes = tracker.minutes.add(1);
            }

            if tracker.minutes > 59 {
                tracker.hours = tracker.hours.add(1);
            }

            if tracker.hours > 23 {
                tracker.seconds = 0;
                tracker.minutes = 0;
                tracker.hours = 0;
            }
        }

        println!("Elapsed time: {:0>2}:{:0>2}:{:0>2}", tracker.hours, tracker.minutes, tracker.seconds);
    }
}
