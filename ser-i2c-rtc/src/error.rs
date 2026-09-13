use nobcd::BcdError;
use esp_idf_svc::sys::EspError;

#[derive(Debug)]
pub enum AppError {
    Bcd(BcdError),
    Esp(EspError),
}

impl From<BcdError> for AppError {
    fn from(err: BcdError) -> Self {
        AppError::Bcd(err)
    }
}

impl From<EspError> for AppError {
    fn from(err: EspError) -> Self {
        AppError::Esp(err)
    }
}
