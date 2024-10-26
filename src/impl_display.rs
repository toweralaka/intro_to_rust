
use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument).
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter)-> fmt::Result{
        let hex_red = if self.red < 16 { format!("0{:X}", self.red)} else { format!("{:X}", self.red)};
        let hex_green = if self.green < 16 { format!("0{:X}", self.green)} else { format!("{:X}", self.green)};
        let hex_blue = if self.blue < 16 { format!("0{:X}", self.blue)} else { format!("{:X}", self.blue)};
        // let rgb = (self.red*65536)+(self.green*256)+self.blue;
        write!(f, "RGB ({}, {}, {}) 0x{}{}{}",
            self.red, self.green, self.blue,
            hex_red, hex_green, hex_blue)

    }
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", color);
        println!("{}", color);
    }
}
