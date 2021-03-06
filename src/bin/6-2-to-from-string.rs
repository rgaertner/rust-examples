use std::string::ToString;
use std::str::FromStr;
use std::num::ParseIntError;

struct Circle {
    radius: i32
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let radius_string = s.trim_left_matches("radius: ");
        let radius = radius_string.parse::<i32>()?;
        Ok( Circle { radius: radius })
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};

    // we got an implementation for from_str, so try to parse from a str
    let circle2 = "radius: 4".parse::<Circle>().unwrap();
    println!("{}", circle2.to_string());

    // see what happens in the error case
    let circle3 = Circle::from_str("Radius: 4");
    println!("{}", circle3.unwrap().to_string());
}
