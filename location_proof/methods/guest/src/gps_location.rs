use std::time::Duration;
use serialport::available_ports;
use serde::Deserialize; 

#[derive(Deserialize, Debug)]
struct IpLocation {
    loc: String,
    city: String,
    country: String,
}

fn try_gps_serial() -> Option<(f64, f64)> {
    let ports = available_ports().ok()?;

    for port_info in ports {
        if let Ok(mut port) = serialport::new(port_info.port_name, 9600)
            .timeout(Duration::from_secs(2))
            .open()
        {
            let mut buffer = vec![0; 1024];
            if let Ok(bytes) = port.read(buffer.as_mut_slice()) {
                let data = String::from_utf8_lossy(&buffer[..bytes]);
                for line in data.lines() {
                    if line.starts_with("$GPGGA") || line.starts_with("$GPRMC") {
                        let parts: Vec<&str> = line.split(',').collect();
                        if parts.len() > 5 {
                            if let (Ok(lat), Ok(lon)) = (
                                parse_nmea_lat(parts[2], parts[3]),
                                parse_nmea_lon(parts[4], parts[5]),
                            ) {
                                return Some((lat, lon));
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

fn parse_nmea_lat(raw: &str, dir: &str) -> Result<f64, std::num::ParseFloatError> {
    let deg = &raw[0..2];
    let min = &raw[2..];
    let val = deg.parse::<f64>()? + min.parse::<f64>()? / 60.0;
    Ok(if dir == "S" { -val } else { val })
}

fn parse_nmea_lon(raw: &str, dir: &str) -> Result<f64, std::num::ParseFloatError> {
    let deg = &raw[0..3];
    let min = &raw[3..];
    let val = deg.parse::<f64>()? + min.parse::<f64>()? / 60.0;
    Ok(if dir == "W" { -val } else { val })
}


// fn fallback_ip_location() -> Option<(f64, f64)> {
//     let res = reqwest::blocking::get("https://ipinfo.io/json").ok()?;
//     let loc: IpLocation = res.json().ok()?;
//     let coords: Vec<&str> = loc.loc.split(',').collect();
//     if coords.len() == 2 {
//         let lat = coords[0].parse().ok()?;
//         let lon = coords[1].parse().ok()?;
//         Some((lat, lon))
//     } else {
//         None
//     }
// }

pub fn get_location() {
    let location = try_gps_serial();

    match location {
        Some((lat, lon)) => {
            println!("Latitude: {}, Longitude: {}", lat, lon);
        }
        None => {
            eprintln!("Failed to get location");
        }
    }
}