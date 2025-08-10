use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::timer::config::Config;
use esp_idf_svc::hal::timer::TimerDriver;

// Developing a Pulse Width Monitor

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let config = Config::new();
    let peripherals = Peripherals::take().unwrap();

    let mut timer1 = TimerDriver::new(peripherals.timer00, &config).unwrap();
    let mut timer2 = TimerDriver::new(peripherals.timer10, &config).unwrap();

    //To setup the hardware, i need to make use of a function generator -  this is a device that allows the generation of square waves - a kind of pulse (/TODO - link to square wave definition)

    // the idea is to use two pins to register the two outputs of the custom wokwi chip (these are inputs to the ESP32).
    //This chip is designed such that one of the waves generates pulses that are 10ms wide, and the other, 25ms wide.

    // in order to achieve the goal of measuring the pulse width, we need to write a logic that determines the elapsed time between every positive edge, as well as that of the negative edge that follows each positive.

    /**
     * the logic would work as follows:
     * 1. for each pin, define two variables to hold its `old` and `current` signal levels
     * 2. assume that the existing (in this case, initial) signal level of each pin is Level::High 
     * 3. poll the current signal level at the input pin
     * 4. if a positive egde (PosEdge) transition occurs, then reset the timer and then update the `old` value to the receieved signal level.
     * 5. if negative edge transition occurs, capture the timer value and update `old` accordingly
     * 6. calculate and print the duration of the pulse
     * 7. go back to step 3 and repeat.
     */ 

    let mut pin1_old: Level = Level::High;
    let mut pin2_old: Level = Level::High;
    
    let mut pin1_current: Level;
    let mut pin2_current: Level;

    let mut timer1_counter: u64 = 
    0;
    let mut timer2_counter: u64 = 0;

    let pin1 = PinDriver::input(peripherals.pins.gpio0).unwrap();
    let pin2 = PinDriver::input(peripherals.pins.gpio1).unwrap();

    // initialize counter values
    timer1.set_counter(0_u64).unwrap();
    timer2.set_counter(0_u64).unwrap();

    timer1.enable(true).unwrap();
    timer2.enable(true).unwrap();

    loop {
        pin1_current = pin1.get_level();
        pin2_current = pin2.get_level();

        // reset on positive edge, capture value on negative edge
        if pin1_old != pin1_current && pin1_current == Level::High {
            timer1.set_counter(0_u64).unwrap();
        }

        if pin1_old != pin1_current && pin1_current == Level::Low {
            timer1_counter = timer1.counter().unwrap();
            pin1_old = pin1_current;
        }

        if pin2_old != pin2_current && pin2_current == Level::High {
            timer2.set_counter(0_u64).unwrap();
        }

        if pin2_old != pin2_current && pin2_current == Level::Low {
            timer2_counter = timer2.counter().unwrap();
            pin2_old = pin2_current;
        }

        println!("Square wave 1 pulse width: {}ms", timer1_counter / 1000);
        println!("Square wave 2 pulse width: {}ms", timer2_counter / 1000);
    }
}
