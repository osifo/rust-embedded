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

    let row_1 = PinDriver::output(peripherals.pins.gpio14.degrade_output())?;
    let row_2 = PinDriver::output(peripherals.pins.gpio13.degrade_output())?;
    let row_3 = PinDriver::output(peripherals.pins.gpio12.degrade_output())?;
    let row_4 = PinDriver::output(peripherals.pins.gpio11.degrade_output())?;

    let col_1 = PinDriver::input(peripherals.pins.gpio10.degrade_input())?;
    let col_2 = PinDriver::input(peripherals.pins.gpio9.degrade_input())?;
    let col_3 = PinDriver::input(peripherals.pins.gpio46.degrade_input())?;

    let mut rows = [row_1, row_2, row_3, row_4];
    let mut columns = [col_1, col_2, col_3];

    let mut last_pressed_key = None;
    let mut pressed_key = None;

    for row in rows {
        row.set_low();
    }

    for col in columns {
        col.set_pull(Pull::Up)?;

        // interrupt is only triggered when the button is pressed (high to low)
        col.set_interrupt_type(InterruptType::FallingEdge)?;

        // subscribe to the interrupt handler
        unsafe { col.subscribe(keypress_handler)?; }

        col.enable_interrupt();
    }

    let keypad_map = [
        ['1', '2', '3'],
        ['4', '5', '6'],
        ['7', '8', '9'],
        ['*', '0', '#'],
    ];

    loop {
        if IS_KEYPRESS_PENDING.swap(false, Ordering::SeqCst) {
            // scan through the keypads to determine what button was pressed

            //prepare for scan
            for row in &mut rows { row.set_level(Level::High); }

            for (row_index, row) in rows.iter().enumerate() {
                row.set_low()?;

                FreeRtos::delay_ms(1); // for electrical stabilization
                for (col_index, col) in columns.iter().enumerate() {
                    if col.is_low() {
                        pressed_key = Some(keypad_map[row_index][col_index]);
                    }
                }

                row.set_high();

                if pressed_key.is_some() {
                    break;
                }
            }

            FreeRtos::delay_ms(20); // this delay is to handle debouncing.

            if let Some(active_key) = pressed_key {
                log::info!("pressed key -------- {}", active_key);
            }

            // handle users' long press
            while columns.iter().any(|c| c.is_low()) {
                FreeRtos::delay_ms(10)
            }

            //reset after scan
            for row in &mut rows { row.set_level(Level::High); }
        }
    }
}

fn keypress_handler () -> Result<(), EspError> {
    IS_KEYPRESS_PENDING.store(true,  Ordering::SeqCst);
    Ok(())
}
