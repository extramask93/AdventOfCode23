use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;

struct Calibration {
    value: u32,
}
impl FromStr for Calibration {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s
            .chars()
            .find_map(|c| c.to_digit(10))
            .expect(&format!("No digit found in {}", s));
        let second = s
            .chars()
            .rev()
            .find_map(|c| c.to_digit(10))
            .expect(&format!("No digit found in {}", s));
        let value = first * 10
            + second;
        Ok(Calibration { value })
    }
}
fn prepare_input(s: &str) -> String {
    s.replace("zero", "z0o")
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

fn main() {
    //read file
    let reader = BufReader::new(File::open("data.txt").expect("cant open the file specified"));
    let total = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| Calibration::from_str(&line).expect("Cant find digit"))
        .fold(0, |val, cal| cal.value + val);
    println!("Total value is: {}", total);
    let reader2 = BufReader::new(File::open("data.txt").expect("cant open the file specified"));
    let total2 = reader2
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| prepare_input(&line))
        .map(|line| Calibration::from_str(&line).expect("Cant find digit"))
        .fold(0, |val, cal| cal.value + val);
    println!("Total value for part 2 is: {}", total2);
}
#[cfg(test)]
mod test {
    use super::{prepare_input,Calibration};
    #[test]
    fn test_calibration_part1() {
        let input_string = r"1abc2
                             pqr3stu8vwx
                             a1b2c3d4e5f
                             treb7uchet";
        let value: u32 = input_string
            .lines()
            .map(|line| line.parse::<Calibration>().unwrap().value)
            .sum();
        assert_eq!(value, 142);
    }
    #[test]
    fn test_calibration_part2() {
        let input_string = r"two1nine
                            eightwothree
                            abcone2threexyz
                            xtwone3four
                            4nineeightseven2
                            zoneight234
                            7pqrstsixteen";
        let value: u32 = input_string
            .lines()
            .map(|line| prepare_input(&line))
            .map(|line| line.parse::<Calibration>().unwrap().value)
            .sum();
        assert_eq!(value, 281);
    }
}
