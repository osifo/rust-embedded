
use esp_idf_svc::hal::gpio;
use esp_idf_svc::hal::delay::BLOCK;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::uart::*;
use esp_idf_svc::hal::units::Hertz;

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let rx = peripherals.pins.gpio22;
    let tx = peripherals.pins.gpio21;

    let uart_config = config::Config::new().baudrate(Hertz(115_200))
        .source_clock(config::SourceClock::Crystal);
    
    let uart = UartDriver::new(
        peripherals.uart1,
        tx,
        rx,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &uart_config
    ).unwrap();


    // set the message plaintext value
    const MESSAGE: &str = "Hello there, this is Osifo";
    const CIPHER_KEY: u8 = 151;
    let mut buferred_input = Vec::new();

    // convert the input string into it's byte array equivalent and encrpyt into a garbled format.
    let garbledmsg: Vec<u8> = MESSAGE.as_bytes().iter().map(|msg| msg ^ CIPHER_KEY).collect();
    

    // transmit the encrypted message over the wire in chunks.
    for chunk in garbledmsg.iter() {
        uart.write(&[*chunk]).unwrap();
        // to receive the transmitted message, I need to make use of the buffer vec to hold the message bytes until I receive a stop bit.
        let mut buf_entry = [0_u8; 1];
        uart.read(&mut buf_entry, BLOCK).unwrap();
        buferred_input.extend_from_slice(&buf_entry);
    }



    log::info!("message received ======= {:?}", buferred_input);


    // now I attempt to ungarble the message, to get it back to plaintext.
    let decoded_msg: Vec<u8> = buferred_input.iter().map(|msg| msg ^ CIPHER_KEY).collect();

    log::info!("I am the decoded message  ====== {:?}", decoded_msg);

    if let Ok(message) = std::str::from_utf8(&decoded_msg) {
        log::info!("Message in human readable encoding (utf-8) ======= \n{:?}", message);
    }

    // loop {}

    log::info!("Hello, world!");
}
