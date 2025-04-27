use thiserror::Error;

#[derive(Error, Debug)]
pub enum LocationFetchError {
    #[error("Failed to read from GPS serial port: {0}")]
    SerialPortError(String),

    #[error("No GPS data available")]
    NoData,
}