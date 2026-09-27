use anyhow::Result;
use esp_idf_svc::wifi::{
    AuthMethod,
    BlockingWifi,
    ClientConfiguration,
    Configuration,
    EspWifi
};

use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::hal::peripherals::Peripherals;
// use esp_idf_svc::sys::EspError;

// use esp_idf_svc::http::client::{
//     Configuration as HttpClientConfig,
//     EspHttpConnection
// };
// use esp_idf_svc::http::Method;
use esp_idf_svc::http::server::{
    Configuration as HttpServerConfig,
    EspHttpServer,
    Method,
};
use std::{thread::sleep, time::Duration};

// use esp_idf_svc::mdns::EspMdns;

const WIFI_SSID: &str = "my outside.co24";
const WIFI_PASSWORD: &str = "tilte_labs_001";

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let _wifi = setup_wifi()?;
    // let _mdns = setup_mdns()?;
    let _server = setup_http_server()?;

    loop {
        sleep(Duration::from_millis(1_000_u64));
    }
}

fn setup_wifi() -> Result<BlockingWifi<EspWifi<'static>>, anyhow::Error> {
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

    wifi.start()?;
    wifi.connect()?;

    wifi.wait_netif_up()?;     

    while !wifi.is_connected()? {
        log::info!("attempting to connect to wifi...");
    }

    let wifi_config = wifi.get_configuration()?;

    if let Configuration::Client(conf) = wifi_config {
        log::info!("wifi connected successfully to {:?}", conf.ssid);
    } else {
        log::info!("wifi connected successfully");
    }
    Ok(wifi)
}

fn setup_http_server<'a>() -> Result<EspHttpServer<'static>, anyhow::Error> {
    let mut http_server = EspHttpServer::new(&HttpServerConfig::default()).unwrap();

    let _ = http_server.fn_handler("/", Method::Get, |request| {
        let html = index_html();
        let mut response = request.into_ok_response()?;

        response.write(html.as_bytes())?;
        // Ok::<String, anyhow::Error>(html);
        Ok::<(), anyhow::Error>(())
    })?;
    Ok(http_server)
}

fn index_html() -> String {
    format!(
        r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="UTF-8">
                    <title>MyHaus - Local Server</title>
                </head>
                <body>
                    <h1>MyHaus - Local Server</h1>
                    <p>Welcome to the your local Haus server!</p>
                </body>
            </html>
        "#
    )
}

// fn setup_mdns() -> Result<EspMdns, anyhow::Error> {
//     let mdns = EspMdns::take()?;
//     mdns.set_hostname("myhuas");
//     mdns.set_instance_name("You Personal MyHaus Dashboard");
//     Ok(mdns)
// }
