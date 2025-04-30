use reqwest::blocking::Client;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct IpApiResponse {
    lat: f64,
    lon: f64,
    city: String,
    country: String,
    query: String, // IP
}

fn get_ip_location() -> Result<IpApiResponse, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .user_agent("LocationFetcher/1.0")
        .build()?;

    let res = client
        .get("http://ip-api.com/json/")
        .send()?
        .error_for_status()? // ensures HTTP 200
        .json::<IpApiResponse>()?;

    Ok(res)
}

fn main() {
    match get_ip_location() {
        Ok(loc) => {
            println!("IP: {}", loc.query);
            println!("City: {}, Country: {}", loc.city, loc.country);
            println!("Latitude: {}, Longitude: {}", loc.lat, loc.lon);
        }
        Err(e) => eprintln!("Failed to fetch location: {}", e),
    }
}
