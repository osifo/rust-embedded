use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::timer::config::Config;
use esp_idf_svc::hal::timer::{TimerDriver, TIMER00, TIMER01, TIMER10, TIMER11};
use rand::Rng;

pub struct Timers {
    pub display_delay: TimerDriver<'static>,
    pub display_duration: TimerDriver<'static>,
    pub response_duration: TimerDriver<'static>,
    pub feedback_delay: TimerDriver<'static>
}

impl Timers {
    //setup timer peripherals
    pub fn new(delay: TIMER00, duration: TIMER01, response: TIMER10, feedback_delay: TIMER11) -> Self {
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
    
    pub fn track_delay_timer(&mut self) {
        let delay_duration: u64 = rand::rng().random_range(MIN_DELAY..=MAX_DELAY);

        self.display_delay.set_counter(0_u64).expect("could not initialize counter to track display duration.");
        self.display_delay.set_alarm(delay_duration).expect("could not set alarm for display timer");
        self.display_delay.enable_alarm(true).expect("could not enable alarm for display timer");
        self.display_delay.enable(true).expect("could not enable timer to track display duration.");
    }


    pub fn track_display_duration(&mut self) {
        let display_duration: u64 = rand::rng().random_range(MIN_DISPLAY_DURATION..=MAX_DISPLAY_DURATION);

        self.display_duration.set_counter(0_u64).expect("could not initialize counter to track display duration.");
        self.display_duration.set_alarm(display_duration).expect("could not set alarm for display timer");
        self.display_duration.enable_alarm(true).expect("could not enable alarm for display timer");
        self.display_duration.enable(true).expect("could not enable timer to track display duration.");
    }

    pub fn track_feedback_timer(&mut self) {
        self.feedback_delay.set_counter(0_u64).expect("could not initialize counter to track display duration.");
        self.feedback_delay.set_alarm(FEEDBACK_DURATION).expect("could not set alarm for display timer");
        self.feedback_delay.enable_alarm(true).expect("could not enable alarm for display timer");
        self.feedback_delay.enable(true).expect("could not enable timer to track display duration.");
    }

    pub fn track_response_speed(&mut self) {
        self.response_duration.set_counter(0_u64).expect("could not initialize counter to track response speed.");
        self.response_duration.enable(true).expect("could not enable timer to track response speed.");
    }

    pub fn get_response_speed_secs(&mut self) -> u64 {
        match self.response_duration.counter() {
            Ok(speed) => speed * 1000,
            Err(_) => 0
        }
    }

    pub fn reset_all(&mut self) {
        let _ = self.display_delay.enable(false);
        let _ = self.display_duration.enable(false);
        let _ = self.response_duration.enable(false);
    }
}