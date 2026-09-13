mod error;

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

    enum DS3231 {
        Seconds,
        Minutes,
        Hours,
        Day,
        Date,
        Month,
        Year
    }

    enum DAY {
        Sun = 1,
        Mon = 2,
        Tue = 3,
        Wed = 4,
        Thu = 5,
        Fri = 6,
        Sat = 7
    }

    struct DateTime {
        sec: u8,
        min: u8,
        hrs: u8,
        day: u8,
        date: u8,
        month: u8,
        year: u8
    }

    let start_date = DateTime {
        sec: 0,
        min: 46,
        hrs: 14,
        day: DAY::Sun as u8,
        date: 13,
        month: 9,
        year: 26,
    };

        
    let mut ds3231_rtc: I2cDriver<'_> = I2cDriver::new(
        i2c,
        sda,
        scl,
        &i2c_config
    ).unwrap();

    // seed the RTC with time data

    let secs: [u8; 1] = BcdNumber::new(start_date.sec)?.bcd_bytes();
    ds3231_rtc.write(DS3231_RTC_ADDR, &[DS3231::Seconds as u8, secs[0]], BLOCK)?;

    let mins: [u8; 1] = BcdNumber::new(start_date.min)?.bcd_bytes();
    ds3231_rtc.write(DS3231_RTC_ADDR, &[DS3231::Minutes as u8, mins[0]], BLOCK)?;

    let hours: [u8; 1] = BcdNumber::new(start_date.hrs)?.bcd_bytes();
    ds3231_rtc.write(DS3231_RTC_ADDR, &[DS3231::Hours as u8, hours[0]], BLOCK)?;

    let day_of_week: [u8; 1] = BcdNumber::new(start_date.day)?.bcd_bytes();
    ds3231_rtc.write(DS3231_RTC_ADDR, &[DS3231::Day as u8, day_of_week[0]], BLOCK)?;

    let day_of_month: [u8; 1] = BcdNumber::new(start_date.date)?.bcd_bytes();
    ds3231_rtc.write(DS3231_RTC_ADDR, &[DS3231::Date as u8, day_of_month[0]], BLOCK)?;
   
    let month: [u8; 1] = BcdNumber::new(start_date.month)?.bcd_bytes();
    ds3231_rtc.write(DS3231_RTC_ADDR, &[DS3231::Month as u8, month[0]], BLOCK)?;
    
    let year: [u8; 1] = BcdNumber::new(start_date.year)?.bcd_bytes();
    ds3231_rtc.write(DS3231_RTC_ADDR, &[DS3231::Year as u8, year[0]], BLOCK)?;
    
    log::info!("Hello, world!");

    loop {
        // this array would hold the data read from the RTC.
        let mut data: [u8; 7] = [0_u8; 7];

        ds3231_rtc.write(DS3231_RTC_ADDR, &[0_u8], BLOCK)?;
        ds3231_rtc.read(DS3231_RTC_ADDR, &mut data, BLOCK)?;

        println!("{:?}", data);

        let secs = BcdNumber::from_bcd_bytes([data[0] & 0x7f])?.value::<u8>(); // added the hex to remove unnecessary data
        let mins = BcdNumber::from_bcd_bytes([data[1]])?.value::<u8>();
        let hours = BcdNumber::from_bcd_bytes([data[2] & 0x3f])?.value::<u8>();
        let date = BcdNumber::from_bcd_bytes([data[4]])?.value::<u8>();
        let month = BcdNumber::from_bcd_bytes([data[5]])?.value::<u8>();
        let year = BcdNumber::from_bcd_bytes([data[6]])?.value::<u8>();

        let day_of_week =  match BcdNumber::from_bcd_bytes([data[3]])?.value::<u8>() {
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
            day_of_week, date, month, year, hours, mins, secs
        );

        FreeRtos::delay_ms(1000_u32);
    }
}
