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
    let ways = records
        .timings
        .iter()
        .zip(records.distances.iter())
        .fold(1, |acc, (time, distance)| {
            acc * RecordTable::get_ways_to_win(*time, *distance)
        });
    println!("{}", ways);
}

struct RecordTable {
    timings: Vec<u32>,
    distances: Vec<u32>,
}
impl RecordTable {
    fn calc_distance(hold_time: u32, total_time: u32) -> u32 {
        hold_time * (total_time - hold_time)
    }
    fn get_ways_to_win(time: u32, record: u32) -> usize {
        (0..=time)
            .map(|hold_time| RecordTable::calc_distance(hold_time, time))
            .filter(|distance| *distance > record)
            .count()
    }
}
fn parse_time(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, timings) = preceded(
        nom::bytes::complete::tag("Time:"),
        preceded(multispace0, separated_list1(multispace1, complete::u32)),
    )(input)?;
    Ok((input, timings))
}
fn parse_distance(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, timings) = preceded(
        nom::bytes::complete::tag("Distance:"),
        preceded(multispace0, separated_list1(multispace1, complete::u32)),
    )(input)?;
    Ok((input, timings))
}

impl FromStr for RecordTable {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (timings, distances)) =
            separated_pair(parse_time, newline, parse_distance)(s).unwrap();
        Ok(RecordTable { timings, distances })
    }
}

#[cfg(test)]
mod test {
    use crate::RecordTable;

    #[test]
    fn load_input() {
        let input = r"Time:      7  15   30
Distance:  9  40  200";
        let records = input.parse::<RecordTable>().unwrap();
        assert_eq!(records.timings.len(), 3);
        assert_eq!(records.distances.len(), 3);
        let ways = records
            .timings
            .iter()
            .zip(records.distances.iter())
            .fold(1, |acc, (time, distance)| {
                acc * RecordTable::get_ways_to_win(*time, *distance)
            });
        assert_eq!(
            RecordTable::get_ways_to_win(records.timings[0], records.distances[0]),
            4
        );
        assert_eq!(
            RecordTable::get_ways_to_win(records.timings[1], records.distances[1]),
            8
        );
        assert_eq!(
            RecordTable::get_ways_to_win(records.timings[2], records.distances[2]),
            9
        );
        assert_eq!(ways, 288);
    }
}
