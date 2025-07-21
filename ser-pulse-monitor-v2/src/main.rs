use esp_idf_svc::hal::timer::TimerDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::timer::config::Config;
use std::sync::atomic::{AtomicBool, Ordering};

static PIN1_SIGNAL_RECEIVED: AtomicBool = AtomicBool::new(false);
static PIN2_SIGNAL_RECEIVED: AtomicBool = AtomicBool::new(false);

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // the goal of this project is to implement a pulse width monitor that measures duration between each eignal (pulse).
    // this v2 updates on the previous by only running the width-measurement logic when a pulse occurred, vs at constant intervals

    // pre-req: using a custom function (pulse) generator connected to two pins on the board.

    // 1.  set the intital signal values (voltage level) of both pins's signals and their counters
    let mut old_pin1_voltage: Level = Level::High;
    let mut old_pin2_voltage: Level = Level::High;

    let timer1_counter: u64 = 0;
    let timer2_counter: u64 = 0;

    //2. determine the two pins to be used, initiatize them
    let peripherals = Peripherals::take().unwrap();
    let pin1 = PinDriver::input(peripherals.pins.gpio1.downgrade_input()).expect("could not setup pin1");
    let pin2 = PinDriver::input(peripherals.pins.gpio2.downgrade_input()).expect("could not setup pin2");

    // 3. set up the counters for measuring duration between signal changes for each pin
    // (i don't have any custom values for the default config struct, so no need to set)
    let timer1 = TimerDriver::new(peripherals.timer00, &Config::new()).expect("could not setup timer for pin 1");
    timer1.set_counter(0_u64);
    timer1.enable(true).expect("could not enable timer1");
    
    let timer2 = TimerDriver::new(peripherals.timer01, &Config::new()).expect("could not setup timer for pin 2");
    timer2.set_counter(0_u64);
    timer2.enable(true).expect("could not enable timer1");

    // 4. setup the pins to trigger an interrupt anytime there's a change to it's voltage level (either +ve or -ve edge)
    pin1.set_interrupt_type(InterruptType::AnyEdge);
    unsafe { pin1.subscribe(reigster_pin1_signal).expect("could not connect pin1 to ISR"); }
    pin1.enable_interrupt().expect("Could not enable interrupt for pin 1");

    pin2.set_interrupt_type(InterruptType::AnyEdge);
    unsafe { pin1.subscribe(reigster_pin2_signal).expect("could not connect pin1 to ISR"); }
    pin2.enable_interrupt().expect("Could not enable interrupt for pin 2");
    
    loop {
        if PIN1_SIGNAL_RECEIVED.load(Ordering::Relaxed) {
           calculate_pulse_duration(pin1, old_pin1_voltage, timer1, timer1_counter);
           PIN1_SIGNAL_RECEIVED.store(false, Ordering::Relaxed); 
        }

        if PIN2_SIGNAL_RECEIVED.load(Ordering::Relaxed) {
            calculate_pulse_duration(pin2, old_pin2_voltage, timer2, timer2_counter);
            PIN2_SIGNAL_RECEIVED.store(false, Ordering::Relaxed);
        }
    }
}

fn calculate_pulse_duration(
        pin: PinDriver<'static, AnyInputPin, Input>, 
        old_signal_value: Level,
        pin_timer: TimerDriver,
        timer_counter: u64) {
    // steps to determine duration between PosEdge+NegEdge

    // if a signal transition occured and is a PosEdge, reset counter and update old signal value
    let current_signal_voltage = pin.get_level();
    
    if old_signal_value != current_signal_voltage && pin.is_set_high() {
        old_signal_value = Level::High;
        pin_timer.set_counter(0_u64);
    } 
    
    // if a signal transition occured, and is NegEdge, capture timer value and update old signal value.
    if old_signal_value != current_signal_voltage && pin.is_set_low() {
        timer_counter = pin_timer.counter().expect("unable to get counter value");
        old_signal_value = Level::Low;

        println!("The duration since last pulse is: {}", timer_counter/1000);
    }
}

fn reigster_pin1_signal() {
    PIN1_SIGNAL_RECEIVED.store(true, Ordering::Relaxed);
}

fn reigster_pin2_signal() {
    PIN2_SIGNAL_RECEIVED.store(true, Ordering::Relaxed);
}