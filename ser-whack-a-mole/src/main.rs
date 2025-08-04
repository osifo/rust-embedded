use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::timer::config::Config;
use esp_idf_svc::hal::timer::{TimerDriver, TIMER00, TIMER01, TIMER10};
use esp_idf_svc::hal::gpio::PinDriver;
use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};
// use esp_idf_svc::hal::delay::FreeRtos;

static IS_BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);
static HAS_DISPLAY_TIMED_OUT: AtomicBool = AtomicBool::new(false);
static HAS_DELAY_TIMED_OUT: AtomicBool = AtomicBool::new(false);

const MIN_DELAY: u64 = 3000;
const MAX_DELAY: u64 = 6000;

const MIN_DISPLAY_DURATION: u64 = 500;
const MAX_DISPLAY_DURATION: u64 = 1500;

struct LedPins {
    pub display: PinDriver<'static, Gpio14, Output>,
    pub success: PinDriver<'static, Gpio12, Output>,
    pub fail: PinDriver<'static, Gpio7, Output>
}

impl LedPins {
    fn new(display_pin: Gpio14, success_pin: Gpio12, failure_pin: Gpio7) -> Self {
        let display = PinDriver::output(display_pin).expect("could not initiate pin for display led");
        let success = PinDriver::output(success_pin).expect("could not initiate pin for success led");
        let fail = PinDriver::output(failure_pin).expect("could not initiate pin for failure led");
    
        LedPins {
            display,
            success,
            fail
        }
    }

    fn reset_all(&mut self) {
        self.display.set_level(Level::Low).expect("could not turn off display led.");
        self.success.set_level(Level::Low).expect("could not turn off success led.");
        self.fail.set_level(Level::Low).expect("could not turn off fail led.");
    }

    fn display_success_feedback(&mut self, response_speed: u64) {
        self.success.set_level(Level::High).expect("could not turn on success led.");
        println!("You responded in {} ms", response_speed);
    }

    fn display_failure_feedback(&mut self) {
        self.fail.set_level(Level::High).expect("could not turn on failure led.");
    }
}

struct Timers {
    pub display_delay: TimerDriver<'static>,
    pub display_duration: TimerDriver<'static>,
    pub response_duration: TimerDriver<'static>
}

impl Timers {
    //setup timer peripherals
    fn new(delay: TIMER00, duration: TIMER01, response: TIMER10) -> Self {
        let timer_config: Config = Config::new().auto_reload(true);

        let mut delay_timer = TimerDriver::new(delay, &timer_config).expect("unable to setup display countdown timer.");
        let mut duration_timer = TimerDriver::new(duration, &timer_config).expect("unable to setup display duration timer.");
        let response_timer = TimerDriver::new(response, &timer_config).expect("unable to setup response timer.");
        
        unsafe { delay_timer.subscribe(handle_delay_timeout).expect("could not setup ISR for delay timer"); }
        delay_timer.enable_interrupt().expect("could not enable interrupt for delay timer.");

        unsafe { duration_timer.subscribe(handle_display_timeout).expect("could not setup ISR for display timer"); }
        duration_timer.enable_interrupt().expect("could not enable interrupt for duration timer.");

        Timers {
            display_delay: delay_timer,
            display_duration: duration_timer,
            response_duration: response_timer
        }
    }
    
    fn track_delay_timer(&mut self) {
        let delay_duration: u64 = rand::rng().random_range(MIN_DELAY..=MAX_DELAY);

        self.display_delay.set_counter(0_u64).expect("could not initialize counter to track display duration.");
        self.display_delay.set_alarm(delay_duration).expect("could not set alarm for display timer");
        self.display_delay.enable_alarm(true).expect("could not enable alarm for display timer");
        self.display_delay.enable(true).expect("could not enable timer to track display duration.");
    }

    fn track_display_duration(&mut self) {
        let display_duration: u64 = rand::rng().random_range(MIN_DISPLAY_DURATION..=MAX_DISPLAY_DURATION);

        self.display_duration.set_counter(0_u64).expect("could not initialize counter to track display duration.");
        self.display_duration.set_alarm(display_duration).expect("could not set alarm for display timer");
        self.display_duration.enable_alarm(true).expect("could not enable alarm for display timer");
        self.display_duration.enable(true).expect("could not enable timer to track display duration.");
    }

    fn track_response_speed(&mut self) {
        self.response_duration.set_counter(0_u64).expect("could not initialize counter to track response speed.");
        self.response_duration.enable(true).expect("could not enable timer to track response speed.");
    }

    fn get_display_duration_secs(&mut self) -> u64 {
        let duration  = self.display_duration.counter().expect("could not get response speed.");
        duration / 10000
    }

    fn get_response_speed_secs(&mut self) -> u64 {
        match self.response_duration.counter() {
            Ok(speed) => speed * 1000,
            Err(_) => 0
        }
    }

    fn reset_all(&mut self) {
        self.track_delay_timer();
        self.track_display_duration();
        self.track_response_speed();
    }
}

fn show_display_led(leds: &mut LedPins, timers: &mut Timers) {
    leds.display.set_level(Level::High).expect("could not turn on display led.");
    timers.track_display_duration();
    timers.track_response_speed();
}

fn restart_game(timers: &mut Timers) {
    HAS_DISPLAY_TIMED_OUT.store(false, Ordering::Relaxed);
    IS_BUTTON_PRESSED.store(false, Ordering::Relaxed);
    timers.track_delay_timer();
}

fn handle_delay_timeout() {
    HAS_DELAY_TIMED_OUT.store(true, Ordering::Relaxed);
}

fn user_button_pressed() {
    IS_BUTTON_PRESSED.store(true, Ordering::Relaxed);
}

fn handle_display_timeout() {
    HAS_DISPLAY_TIMED_OUT.store(true, Ordering::Relaxed);
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // setup gpio peripherals 
    let peripherals = Peripherals::take().expect("unable to instantiate peripherals");
    
    let mut leds  = LedPins::new(
        peripherals.pins.gpio14,
        peripherals.pins.gpio12,
        peripherals.pins.gpio7
    );

    let mut timers: Timers = Timers::new(
        peripherals.timer00, 
        peripherals.timer01, 
        peripherals.timer10
    );
    
    let mut button = PinDriver::input(peripherals.pins.gpio21).expect("could not initiate pin for button");
    button.set_pull(Pull::Up).expect("could not set pull type for button");
    button.set_interrupt_type(InterruptType::PosEdge).expect("could not set interrupt type");

    
    unsafe { button.subscribe(user_button_pressed).expect("could not connect the button to the ISR"); }
    button.enable_interrupt().expect("could not enable interrupt");
    
    // connect an ISR to the button for button-press events


    timers.track_delay_timer();

    loop {
        let button_was_pressed = IS_BUTTON_PRESSED.load(Ordering::Relaxed);
        let display_timed_out: bool = HAS_DISPLAY_TIMED_OUT.load(Ordering::Relaxed);

        if HAS_DELAY_TIMED_OUT.load(Ordering::Relaxed) {
            leds.reset_all();
            show_display_led(&mut leds, &mut timers);
            HAS_DELAY_TIMED_OUT.store(false, Ordering::Relaxed);
        }

        if button_was_pressed && !display_timed_out {
            let response_speed: u64 = timers.get_response_speed_secs();
            leds.display_success_feedback(response_speed);
            restart_game(&mut timers);
        }

        if display_timed_out {
            leds.display_failure_feedback();
            restart_game(&mut timers);
        }
    }
}
