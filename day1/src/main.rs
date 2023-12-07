use std::fs::File;
use std::io::{self, prelude::*,BufReader};
use std::str::FromStr;


struct Calibration {
    value: i32
}
impl FromStr for Calibration {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.chars().filter(|c| c.to_ascii_lowercase() as u8 <= 57
                                         && c.to_ascii_lowercase() as u8 >= 48).next().unwrap();
        let second = s.chars().rev().filter(|c| c.to_ascii_lowercase() as u8 <= 57
                                         && c.to_ascii_lowercase() as u8 >= 48).next().unwrap();
        let value = ((first as i32 -48) * 10)+((second as i32 - 48));
        println!("{}", value);
        Ok(Calibration {value})

    }
}

fn main() {
    //read file 
    let mut f = File::open("data.txt").expect("cant open the file specified");
    let reader = BufReader::new(f);
    let total = reader.lines().map(|line| line.unwrap()).map(|line| {
        Calibration::from_str(&line).expect("Cant find digit")
    }).fold(0, |val,cal| cal.value + val);
    println!("Total value is: {}",total);
}

mod test {
#[test]
fn dupa() {

}
}
