use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::timer::config::Config;
use esp_idf_svc::hal::timer::{TimerDriver, TIMER00, TIMER01, TIMER10, TIMER11};
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::delay::FreeRtos;
use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};

static IS_BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);
static HAS_DISPLAY_TIMED_OUT: AtomicBool = AtomicBool::new(false);
static HAS_DELAY_TIMED_OUT: AtomicBool = AtomicBool::new(false);
static IS_FEEDBACK_ACTIVE: AtomicBool = AtomicBool::new(false);
static HAS_GAME_STARTED: AtomicBool = AtomicBool::new(false);

const MIN_DELAY: u64 = 3_000_000;
const MAX_DELAY: u64 = 6_000_000;

const MIN_DISPLAY_DURATION: u64 = 2_000_000;
const MAX_DISPLAY_DURATION: u64 = 6_000_000;

const FEEDBACK_DURATION: u64 = 2_000_000; // has to be less than the min delay

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

    fn reset_all_feedback(&mut self) {
        self.success.set_level(Level::Low).expect("could not turn off success led.");
        self.fail.set_level(Level::Low).expect("could not turn off fail led.");
    }

    fn display_success_feedback(&mut self, response_speed: u64, timers: &mut Timers) {
        self.display.set_level(Level::Low).expect("could not turn off display led."); // turns off display led
        self.success.set_level(Level::High).expect("could not turn on success led.");
        
        IS_FEEDBACK_ACTIVE.store(true, Ordering::Relaxed);
        timers.track_feedback_timer();

        println!("You responded in {} ms", response_speed);

        FreeRtos::delay_ms(100); // delay to values set can take effect
    }

    fn display_failure_feedback(&mut self, timers: &mut Timers) {
        self.display.set_level(Level::Low).expect("could not turn off display led."); // turns off display led
        self.fail.set_level(Level::High).expect("could not turn on failure led.");
        
        IS_FEEDBACK_ACTIVE.store(true, Ordering::Relaxed);
        timers.track_feedback_timer();

        FreeRtos::delay_ms(100); // delay to values set can take effect
    }
}

struct Timers {
    pub display_delay: TimerDriver<'static>,
    pub display_duration: TimerDriver<'static>,
    pub response_duration: TimerDriver<'static>,
    pub feedback_delay: TimerDriver<'static>
}

impl Timers {
    //setup timer peripherals
    fn new(delay: TIMER00, duration: TIMER01, response: TIMER10, feedback_delay: TIMER11) -> Self {
        let timer_config: Config = Config::new();

        let mut delay_timer = TimerDriver::new(delay, &timer_config).expect("unable to setup display countdown timer.");
        let mut duration_timer = TimerDriver::new(duration, &timer_config).expect("unable to setup display duration timer.");
        let mut feedback_delay_timer: TimerDriver<'_> = TimerDriver::new(feedback_delay, &timer_config).expect("unable to setup response timer.");
        let response_timer = TimerDriver::new(response, &timer_config).expect("unable to setup response timer.");
        
        unsafe { delay_timer.subscribe(handle_delay_timeout).expect("could not setup ISR for delay timer"); }
        delay_timer.enable_interrupt().expect("could not enable interrupt for delay timer.");
        delay_timer.enable(true).expect("could not enable delay timer");

        unsafe { duration_timer.subscribe(handle_display_timeout).expect("could not setup ISR for display timer"); }
        duration_timer.enable_interrupt().expect("could not enable interrupt for duration timer.");
        duration_timer.enable(true).expect("could not enable response timer");

        unsafe { feedback_delay_timer.subscribe(handle_feedback_timeout).expect("could not setup ISR for feedback timer"); }
        feedback_delay_timer.enable_interrupt().expect("could not enable interrupt for feedback timer.");
        feedback_delay_timer.enable(true).expect("could not enable feedback timer");

        Timers {
            display_delay: delay_timer,
            display_duration: duration_timer,
            response_duration: response_timer,
            feedback_delay: feedback_delay_timer
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

    fn track_feedback_timer(&mut self) {
        self.feedback_delay.set_counter(0_u64).expect("could not initialize counter to track display duration.");
        self.feedback_delay.set_alarm(FEEDBACK_DURATION).expect("could not set alarm for display timer");
        self.feedback_delay.enable_alarm(true).expect("could not enable alarm for display timer");
        self.feedback_delay.enable(true).expect("could not enable timer to track display duration.");
    }

    fn track_response_speed(&mut self) {
        self.response_duration.set_counter(0_u64).expect("could not initialize counter to track response speed.");
        self.response_duration.enable(true).expect("could not enable timer to track response speed.");
    }

    fn get_response_speed_secs(&mut self) -> u64 {
        match self.response_duration.counter() {
            Ok(speed) => speed * 1000,
            Err(_) => 0
        }
    }

    fn reset_all(&mut self) {
        let _ = self.display_delay.enable(false);
        let _ = self.display_duration.enable(false);
        let _ = self.response_duration.enable(false);
        // let _ =  self.feedback_delay.enable(false);
    }
}

fn show_display_led(leds: &mut LedPins, timers: &mut Timers) {
    HAS_DELAY_TIMED_OUT.store(false, Ordering::Relaxed);
    leds.reset_all_feedback();

    leds.display.set_level(Level::High).expect("could not turn on display led.");

    timers.track_display_duration();
    timers.track_response_speed();
}

fn restart_game(leds: &mut LedPins, timers: &mut Timers, button: &mut PinDriver<'_, Gpio21, Input>) {
    timers.reset_all();

    button.enable_interrupt().expect("could not re-enable button interrupt");
    HAS_DELAY_TIMED_OUT.store(false, Ordering::Relaxed);
    HAS_GAME_STARTED.store(false, Ordering::Relaxed);
    HAS_DISPLAY_TIMED_OUT.store(false, Ordering::Relaxed);
    IS_BUTTON_PRESSED.store(false, Ordering::Relaxed);

    timers.track_delay_timer();
}

fn handle_delay_timeout() {
    HAS_DELAY_TIMED_OUT.store(true, Ordering::Relaxed);
    HAS_GAME_STARTED.store(true, Ordering::Relaxed);
}

fn handle_button_pressed() {
    if HAS_GAME_STARTED.load(Ordering::Relaxed) {
        IS_BUTTON_PRESSED.store(true, Ordering::Relaxed);
    }
}

fn handle_display_timeout() {
    HAS_DISPLAY_TIMED_OUT.store(true, Ordering::Relaxed);
}

fn handle_feedback_timeout() {
    IS_FEEDBACK_ACTIVE.store(false, Ordering::Relaxed);
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
        peripherals.timer10,
        peripherals.timer11
    );
    
    let mut button = PinDriver::input(peripherals.pins.gpio21).expect("could not initiate pin for button");
    button.set_pull(Pull::Up).expect("could not set pull type for button");
    button.set_interrupt_type(InterruptType::PosEdge).expect("could not set interrupt type");

    unsafe { button.subscribe(handle_button_pressed).expect("could not connect the button to the ISR"); }
    button.enable_interrupt().expect("could not enable isr for button");

    timers.track_delay_timer();

    loop {
        let button_was_pressed: bool = IS_BUTTON_PRESSED.load(Ordering::Relaxed);
        let display_timed_out: bool = HAS_DISPLAY_TIMED_OUT.load(Ordering::Relaxed);
        let is_displaying_feedback: bool = IS_FEEDBACK_ACTIVE.load(Ordering::Relaxed);
        let new_game_started: bool = HAS_DELAY_TIMED_OUT.load(Ordering::Relaxed);

        // this ensures that button press inputs are only calid after display led has turned on
        let game_ongoing: bool = HAS_GAME_STARTED.load(Ordering::Relaxed);

        if new_game_started && !is_displaying_feedback && !display_timed_out {
            show_display_led(&mut leds, &mut timers);
        }

        if game_ongoing && button_was_pressed && !display_timed_out && !is_displaying_feedback {
            let response_speed: u64 = timers.get_response_speed_secs();
            leds.display_success_feedback(response_speed, &mut timers);
        }
        
        if !button_was_pressed && display_timed_out && !is_displaying_feedback {
            // user fails round
            leds.display_failure_feedback(&mut timers);
        }


        if  !is_displaying_feedback && (button_was_pressed || display_timed_out) {
            // leds.reset_all_feedback();
            restart_game(&mut leds, &mut timers, &mut button);
        }
    }
}
