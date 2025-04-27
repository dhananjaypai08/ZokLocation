mod gps_location;
mod errors;
use gps_location::get_location;
fn main() {
    println!("Hello, world!");
    get_location();
}
