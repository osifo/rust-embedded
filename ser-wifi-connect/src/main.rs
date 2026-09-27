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

use esp_idf_svc::http::client::{
    Configuration as HttpConfig,
    EspHttpConnection
};
use esp_idf_svc::http::Method;


const WIFI_SSID: &str = "my outside.co24";
const WIFI_PASSWORD: &str = "tilte_labs_001";
// const WIFI_SSID: &str = "<wifi name>";
// const WIFI_PASSWORD: &str = "<password>";

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
    authenticate_device();

    // Ok(())

    loop {
        std::thread::sleep(Duration::from_secs(10));
    } 
}

fn authenticate_device() -> Result<(), EspError> {
    // setup http client
    let base_url: &str = "http://192.168.1.205:3000/api/v1";
    let request_endpoint: &str = &String::from(format!("{}/users", base_url));

    let mut http_connection = EspHttpConnection::new(&HttpConfig {
        use_global_ca_store:  true,
        crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;

    let _request = http_connection.initiate_request(
        Method::Get,
        request_endpoint,
        &[]
    );

    let _response  = http_connection.initiate_response()?;
    let response_status = http_connection.status();

    log::info!("===== response status is {} ======= \n", response_status);

    Ok(())

}
