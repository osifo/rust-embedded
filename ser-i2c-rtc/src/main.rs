mod error;
mod date_time;

use esp_idf_svc::hal::delay::{FreeRtos, BLOCK};
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::units::KiloHertz;
use esp_idf_svc::hal::gpio::*;
use nobcd::BcdNumber;

use error::AppError;


const DS3231_RTC_ADDR: u8 = 0x68; // the fixed address of the RTC on the SoC.

fn main() -> Result<(), AppError> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    
    let i2c = peripherals.i2c0; // this is the dedicated circuit on the ESP32 board for i2c
    let sda = peripherals.pins.gpio10;
    let scl  = peripherals.pins.gpio11;

    let i2c_config = I2cConfig::new().baudrate(KiloHertz::from(100).into());
        // .source_clock(config::SourceClock::Crystal);

    let mut ds3231_rtc: I2cDriver<'_> = I2cDriver::new(
        i2c,
        sda,
        scl,
        &i2c_config
    ).unwrap();

    let seed_timestamp = date_time::DateTime {
        sec: 0,
        min: 0,
        hrs: 0,
        day: date_time::DAY::Tue as u8,
        date: 15,
        month: 9,
        year: 26
    };

    let mut seed_buf: [u8; 8] = [0_u8; 8];
    
    for (idx, &field) in seed_timestamp.as_fields().iter().enumerate() {
        let bcd_value: [u8; 1] = BcdNumber::new(field)?.bcd_bytes();
        seed_buf[idx + 1] = bcd_value[0];
    }
    ds3231_rtc.write(DS3231_RTC_ADDR, &seed_buf, BLOCK)?;


    // Bits to mask off per register: seconds has a clock-halt flag in bit 7,
    // hours has 12/24-hour mode + AM/PM bits. Everything else is a no-op mask.
    const MASKS: [u8; 7] = [0x7f, 0xff, 0x3f, 0xff, 0xff, 0xff, 0xff];
    loop {
        // this array would hold the data read from the RTC.
        let mut data_buf: [u8; 7] = [0_u8; 7];

        ds3231_rtc.write_read(DS3231_RTC_ADDR, &[date_time::DS3231::Seconds as u8], &mut data_buf, BLOCK);

        println!("{:?}", data_buf);
        
        let mut current_date = [0_u8; 7];
        for (idx, value) in data_buf.iter().enumerate() {
            current_date[idx] = BcdNumber::from_bcd_bytes([value & MASKS[idx]])?.value::<u8>();
        }
        let [secs, mins, hrs, day, date, month, year] = current_date;


        let day_of_week: &str =  match day {
            1 => "Sunday",
            2 => "Monday",
            3 => "Tuesday",
            4 => "Wednesday",
            5 => "Thursday",
            6 => "Friday",
            7 => "Saturday",
            _ => "invalid date",
        };

        println!(
            "{} {}/{}/20{} {:02}:{:02}:{:02}", 
            day_of_week, date, month, year, hrs, mins, secs
        );

        FreeRtos::delay_ms(1000_u32);
    }
}
