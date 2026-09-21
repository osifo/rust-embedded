use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::sys::EspError;
use std::time::Duration;

use esp_idf_svc::wifi::{
    AuthMethod,
    BlockingWifi,
    ClientConfiguration,
    Configuration,
    EspWifi,
};

const WIFI_SSID: &str = "<wifi name>";
const WIFI_PASSWORD: &str = "<password>";

fn main() -> Result<(), EspError> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();


    // setup the wifi
    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs))?,
        sysloop,
    )?;

    wifi.set_configuration(
        &Configuration::Client(
            ClientConfiguration {
                ssid: WIFI_SSID.try_into().unwrap(),
                password: WIFI_PASSWORD.try_into().unwrap(),
                auth_method: AuthMethod::WPA2Personal,
                ..Default::default()
            }
        )
    )?;

    // connect to the wifi
    log::info!("connecting to wifi...");
    wifi.start()?;
    wifi.connect()?;

    wifi.wait_netif_up()?;

    while !wifi.is_connected()? {
        let config = wifi.get_configuration()?;
        log::info!("Waiting for a client to connect...\n{:?}", config);
    }

    log::info!("Wifi is now connected.");

    // Ok(())

    loop {
        std::thread::sleep(Duration::from_secs(10));
    } 
}
