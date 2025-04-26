use risc0_zkvm::guest::env;
use sha2::{Digest, Sha256};

mod gps_location;
use gps_location::get_location;

fn main() {
    let (latitude1, longitude1, latitude2, longitude2, threshold): (f64, f64, f64, f64, f64) = env::read();
    get_location();
    let dx = latitude1 - latitude2;
    let dy = longitude1 - longitude2;
    let distance = (dx * dx + dy * dy).sqrt();
    assert!(distance <= threshold);
    let mut hasher = Sha256::new();
    hasher.update(latitude1.to_string());
    hasher.update(longitude1.to_string());
    hasher.update(latitude2.to_string());
    hasher.update(longitude2.to_string());
    hasher.update(threshold.to_string());
    let hash = hasher.finalize();
    let hash_array: [u8; 32] = hash.into();
    if distance <= threshold {
        println!("Location is within the threshold");
    } else{
        println!("Location is outside the threshold");
    }

    env::commit(&hash_array);
}
