use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::hal::delay::FreeRtos;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();
    
    let device_p = Peripherals::take().unwrap();

    let pin_1 = PinDriver::output(device_p.pins.gpio1.downgrade_output()).unwrap();
    let pin_2 = PinDriver::output(device_p.pins.gpio18.downgrade_output()).unwrap();
    let pin_3 = PinDriver::output(device_p.pins.gpio6.downgrade_output()).unwrap();

    let mut pin_list = [pin_1, pin_2, pin_3];
    let mut iterated_items = 0;
    let mut current_level = Level::High;
    let led_size = pin_list.len();
    loop {
        while iterated_items < led_size {
            for pin in &mut pin_list {
                iterated_items += 1;
                pin.set_level(current_level).unwrap();

                if iterated_items == led_size {
                    if current_level == Level::High { 
                        current_level = Level::Low;
                    } else {
                        current_level = Level::High;
                    }

                    iterated_items = 0;
                }

                FreeRtos::delay_ms(1000u32)
            }
            
            pin_list.reverse();
        }
    }
}
