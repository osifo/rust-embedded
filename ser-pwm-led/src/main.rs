use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::ledc::{
    config::TimerConfig,
    LedcDriver,
    LedcTimerDriver,
    Resolution
};
use esp_idf_svc::hal::prelude::*;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    //configure teh Ledc timer driver
    let timer_driver = LedcTimerDriver::new(
        peripherals.ledc.timer0, 
        &TimerConfig::default()
            .frequency(1000.Hz())
            .resolution(Resolution::Bits14)
    ).unwrap();

    //configure the PWM driver for generating the signals
    let mut pwm_driver = LedcDriver::new(
        peripherals.ledc.channel0, 
        timer_driver,
        peripherals.pins.gpio7
    ).unwrap();

    // next i need to handle the range and starting intensity (duty cycle).
    let max_duty_cycle = pwm_driver.get_max_duty();
    let start_duty_cycle = 0;

    log::info!("max duty cycle is: {}", max_duty_cycle);

    //initialize the duty cycle.
    pwm_driver.set_duty(start_duty_cycle).unwrap();

    //enable the pwm driver
    pwm_driver.enable().unwrap();

    

    // next is to configure the fading adn reverse-fading loops.

    loop {
        for duty in start_duty_cycle..max_duty_cycle {
            pwm_driver.set_duty(duty).unwrap();
            Ets::delay_us(100);
        }
    
        for reverse_duty in max_duty_cycle..start_duty_cycle {
            pwm_driver.set_duty(reverse_duty).unwrap();
            Ets::delay_us(100);
        }
    }
}
