use std::sync::atomic::{AtomicBool, Ordering};

use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::sys::EspError;
use esp_idf_svc::hal::delay::FreeRtos;

static IS_KEYPRESS_PENDING: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), EspError> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();


    // connect peripherals to gpio pins
    let peripherals = Peripherals::take()?;

    let mut key_combination: String = String::new();


    let row_1 = PinDriver::output(peripherals.pins.gpio14.degrade_output())?;
    let row_2 = PinDriver::output(peripherals.pins.gpio13.degrade_output())?;
    let row_3 = PinDriver::output(peripherals.pins.gpio12.degrade_output())?;
    let row_4 = PinDriver::output(peripherals.pins.gpio11.degrade_output())?;

    let col_1 = PinDriver::input(peripherals.pins.gpio10.degrade_input(), Pull::Up)?;
    let col_2 = PinDriver::input(peripherals.pins.gpio9.degrade_input(), Pull::Up)?;
    let col_3 = PinDriver::input(peripherals.pins.gpio46.degrade_input(), Pull::Up)?;

    let mut rows = [row_1, row_2, row_3, row_4];
    let mut columns = [col_1, col_2, col_3];

    for  row in &mut rows {
        row.set_low()?;
    }

    for col in &mut columns {
        // col.set_pull(Pull::Up)?;

        // interrupt is only triggered when the button is pressed (high to low)
        col.set_interrupt_type(InterruptType::NegEdge)?;

        // subscribe to the interrupt handler
        unsafe { col.subscribe(keypress_handler)?; }

        col.enable_interrupt()?
    }

    let keypad_map = [
        ["1", "2", "3"],
        ["4", "5", "6"],
        ["7", "8", "9"],
        ["CLR", "0", "ENTER"],
    ];

    loop {
        if IS_KEYPRESS_PENDING.swap(false, Ordering::SeqCst) {
            FreeRtos::delay_ms(20); // this delay is to handle physical button bounce.
            let mut pressed_key = None;
            // scan through the keypads to determine what button was pressed

            //prepare for scan
            for row in &mut rows { row.set_level(Level::High)?; }
            for col in &mut columns { col.disable_interrupt()?; }

            for (row_index, row) in rows.iter_mut().enumerate() {
                row.set_low()?;

                FreeRtos::delay_ms(1); // for electrical stabilization
                for (col_index, col) in columns.iter_mut().enumerate() {
                    if col.is_low() {
                        pressed_key = Some(keypad_map[row_index][col_index]);
                        // break;
                    }
                    col.enable_interrupt()?;
                }

                row.set_high()?;

                if pressed_key.is_some() {
                    break;
                }
            }

            if let Some(active_key) = pressed_key {

                match active_key {
                    "CLR" => {
                        log::info!("input cleared. enter pin.");
                        key_combination.clear();
                    },
                    "ENTER" => {
                        log::info!("entered pin is {}", key_combination);
                        key_combination.clear();
                    },
                    _ => {
                        key_combination.push_str(active_key);
                        log::info!("pressed key -------- {}", active_key);
                    }
                }
                
                // handle users' long press
                while columns.iter().any(|c| c.is_low()) {
                    FreeRtos::delay_ms(200);
                }
            }
            
            //reset after scan
            for row in &mut rows { row.set_low()?; }
            for col in &mut columns { col.enable_interrupt()?; }

        }

        FreeRtos::delay_ms(50); // this delay is to prevent the main loop from consuming too much CPU resources.
    }
}

fn keypress_handler () -> () {
    IS_KEYPRESS_PENDING.store(true,  Ordering::SeqCst);
}
