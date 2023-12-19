use std::str::FromStr;

use nom::{
    character::complete::{self, multispace0, multispace1, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let buffer = std::fs::read_to_string("data.txt").unwrap();
    let records = RecordTable::from_str(&buffer).unwrap();
    println!("{}", RecordTable::get_ways_to_win(records.timing, records.distance));
}

struct RecordTable {
    timing: u64,
    distance: u64,
}
impl RecordTable {
    fn calc_distance(hold_time: u64, total_time: u64) -> u64 {
        hold_time * (total_time - hold_time)
    }
    fn get_ways_to_win(time: u64, record: u64) -> usize {
        (0..=time)
            .map(|hold_time| RecordTable::calc_distance(hold_time, time))
            .filter(|distance| *distance > record)
            .count()
    }
}
fn parse_time(input: &str) -> IResult<&str, u64> {
    let (input, timings) = preceded(
        nom::bytes::complete::tag("Time:"),
        preceded(multispace0, separated_list1(multispace1, complete::u64)),
    )(input)?;
    let timing = timings.iter()
        .fold(String::new(), |mut acc,val| {acc.push_str(&val.to_string()); return acc;})
        .parse::<u64>().unwrap();
    Ok((input, timing))
}
fn parse_distance(input: &str) -> IResult<&str, u64> {
    let (input, distances) = preceded(
        nom::bytes::complete::tag("Distance:"),
        preceded(multispace0, separated_list1(multispace1, complete::u64)),
    )(input)?;
    let distance = distances.iter()
        .fold(String::new(), |mut acc,val| {acc.push_str(&val.to_string()); return acc;})
        .parse::<u64>().unwrap();
    Ok((input, distance))
}

impl FromStr for RecordTable {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (timing, distance)) =
            separated_pair(parse_time, newline, parse_distance)(s).unwrap();
        Ok(RecordTable { timing, distance })
    }
}

#[cfg(test)]
mod test {
    use crate::RecordTable;

    #[test]
    fn load_input() {
        let input = r"Time:      7  15   30
Distance:  9  40  200";
        let record = input.parse::<RecordTable>().unwrap();
        assert_eq!(record.timing, 71530);
        assert_eq!(record.distance, 940200);
        let result = RecordTable::get_ways_to_win(record.timing, record.distance);
        assert_eq!(
            result,
            71503
        );
    }
}
