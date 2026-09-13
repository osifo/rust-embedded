use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::log::EspLogger;


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();

    //access the PAC
    let device_p = Peripherals::take().unwrap();

    let row1 = PinDriver::output(device_p.pins.gpio47.downgrade_output()).unwrap();
    let row2 = PinDriver::output(device_p.pins.gpio48.downgrade_output()).unwrap();
    let row3 = PinDriver::output(device_p.pins.gpio45.downgrade_output()).unwrap();
    let row4 = PinDriver::output(device_p.pins.gpio0.downgrade_output()).unwrap();

    let col1 = PinDriver::input(device_p.pins.gpio18.downgrade_input()).unwrap();
    let col2 = PinDriver::input(device_p.pins.gpio17.downgrade_input()).unwrap();
    let col3 = PinDriver::input(device_p.pins.gpio16.downgrade_input()).unwrap();
    let col4 = PinDriver::input(device_p.pins.gpio15.downgrade_input()).unwrap();

    let keypad_map: [[char; 4]; 4] = [
        ['1', '2', '3', 'A'],
        ['4', '5', '6', 'B'],
        ['7', '8', '9', 'C'],
        ['*', '0', '#', 'D'],
    ];

    let mut rows = [row1, row2, row3, row4];
    let cols = [col1, col2, col3, col4];

    loop {
        let mut input_value = '_';

        for (row_index, row) in &mut rows.iter_mut().enumerate() {
            row.set_level(Level::High).unwrap();

            for (col_index, col) in cols.iter().enumerate() {
                if col.is_low() {
                    input_value = keypad_map[row_index][col_index];
                }
            }
            row.set_level(Level::Low).unwrap();
        }

        println!("Key pressed ===== {}", input_value);
    }

}
