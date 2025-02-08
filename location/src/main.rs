enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64), // latitude, longitude
}

impl Location {
    fn display(&self) {
        match *self {
            Location::Unknown => println!("Unknown location!"),
            Location::Anonymous => println!("Anonymous location"),
            Location::Known(lat, lon) => println!("{}, {} ", lat, lon),
        }
    }
}

fn main() {
    let address = Location::Known(28.60, -80.60);
    address.display();
}
