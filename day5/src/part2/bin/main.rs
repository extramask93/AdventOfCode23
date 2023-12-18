use std::io::{BufRead, BufReader, Read};

use nom::{
    bytes::complete::{is_not, tag, take_while},
    character::complete::{self, multispace0, multispace1},
    combinator::opt,
    multi::separated_list1,
    sequence::{pair, preceded, terminated},
    IResult,
};

fn parse_seed_list(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let (input, seeds) = preceded(
        terminated(tag("seeds:"), complete::space0),
        separated_list1(
            complete::space1,
            pair(complete::u64, preceded(complete::space1, complete::u64)),
        ),
    )(input)?;
    Ok((input, seeds))
}
fn parse_maping_entry(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, numbers) = separated_list1(complete::space1, complete::u64)(input)?;
    Ok((input, numbers))
}
fn parse_maping_title(input: &str) -> IResult<&str, &str> {
    let (input, name) = terminated(is_not(" "), complete::multispace0)(input)?;
    let (input, _) = take_while(|a: char| a != '\n')(input)?;
    let (input, _) = opt(multispace0)(input)?;
    Ok((input, name))
}
fn parse_maping(input: &str) -> IResult<&str, Map> {
    let mut result = Map::default();
    let (input, name) = parse_maping_title(input)?;
    let (input, entries) = separated_list1(complete::line_ending, parse_maping_entry)(input)?;
    result.name = name;
    result = entries.iter().fold(result, |mut r, elem| {
        r.elems.push(Mapping {
            from: elem[1],
            to: elem[0],
            number: elem[2],
        });
        r
    });
    Ok((input, result))
}
fn mappings(input: &str) -> IResult<&str, Vec<Map>> {
    let (_, blocks) = separated_list1(
        pair(complete::line_ending, complete::line_ending),
        parse_maping,
    )(input)?;
    Ok((input, blocks))
}
fn main() {
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut buff: String = String::new();
    let _ = file.read_to_string(&mut buff).unwrap();
    let almanac = Almanac::from_str(&buff);
    for i in (0..u64::max_value()) {
        match almanac.location_to_seed(i) {
            Some(seed) => {
                println!("seed {}: location {}", seed, i);
                break;
            }
            None => (),
        }
    }
}
struct Almanac<'a> {
    seeds: Vec<(u64, u64)>,
    maps: Vec<Map<'a>>,
}

impl<'a> Almanac<'a> {
    pub fn from_str(input: &'a str) -> Self {
        let (input, seeds) = parse_seed_list(input).unwrap();
        let (input, _) = multispace1::<&str, nom::error::Error<&str>>(input).unwrap();
        let (_, maps) = mappings(input).unwrap();
        let result = Almanac { seeds, maps };
        result
    }
    fn has_seed(&self, seed: u64) -> bool {
        self.seeds
            .iter()
            .find(|(begin, elems)| seed >= *begin && seed < *begin + *elems)
            .is_some()
    }
    fn location_to_seed(&self, location: u64) -> Option<u64> {
        let seed = self
            .maps
            .iter()
            .rev()
            .fold(location, |loc, mapping| mapping.destination_to_source(loc));
        if self.has_seed(seed) {
            return Some(seed);
        }
        None
    }
}
#[derive(Default, Debug)]
struct Map<'a> {
    name: &'a str,
    elems: Vec<Mapping>,
}
impl Map<'_> {
    fn destination_to_source(&self, destination: u64) -> u64 {
        self.elems
            .iter()
            .find_map(|maping| maping.to(destination))
            .or_else(|| Some(destination))
            .expect("Map must be some")
    }
}
#[derive(Debug)]
struct Mapping {
    from: u64,
    to: u64,
    number: u64,
}
impl Mapping {
    fn to(&self, t: u64) -> Option<u64> {
        if t < self.to || t > self.to + self.number {
            return None;
        }
        return Some(self.from + (t - self.to));
    }
}
#[cfg(test)]
mod test {
    use crate::{
        mappings, parse_maping, parse_maping_entry, parse_maping_title, parse_seed_list, Almanac,
    };

    #[test]
    fn parse_seeds() {
        let input = "seeds: 79 14 55 13";
        let (_, seeds) = parse_seed_list(&input).unwrap();
        assert_eq!(seeds, vec![(79, 14), (55, 13)]);
    }
    #[test]
    fn parse_title() {
        let input = r"seed-to-soil map:";
        assert_eq!(parse_maping_title(&input).unwrap().1, "seed-to-soil");
    }
    #[test]
    fn parse_entry() {
        let input = r"45 77 23";
        assert_eq!(parse_maping_entry(&input).unwrap().1, vec![45, 77, 23]);
    }
    #[test]
    fn parse_block() {
        let input = r"seed-to-soil map:
50 98 2
52 50 48";
        let block = parse_maping(input).unwrap().1;
        assert_eq!(block.name, "seed-to-soil");
        assert_eq!(block.elems.len(), 2);
        assert_eq!(block.source_to_destination(79), 81);
        assert_eq!(block.destination_to_source(81), 79);
        assert_eq!(block.source_to_destination(14), 14);
        assert_eq!(block.destination_to_source(14), 14);
        assert_eq!(block.source_to_destination(55), 57);
        assert_eq!(block.destination_to_source(57), 55);
        assert_eq!(block.source_to_destination(13), 13);
        assert_eq!(block.destination_to_source(13), 13);
    }
    #[test]
    fn parse_multiple_block() {
        let input = r"seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let mappings = mappings(input).unwrap().1;
        assert_eq!(mappings.len(), 7);
    }

    #[test]
    fn parse_whole_data() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let almanac = Almanac::from_str(input);
        assert_eq!(almanac.seeds, vec![(79, 14), (55, 13)]);
        assert_eq!(almanac.maps.len(), 7);
        assert_eq!(almanac.has_seed(79), true);
        for i in (0..u64::max_value()) {
            match almanac.location_to_seed(i) {
                Some(seed) => {
                    assert_eq!(seed, 82);
                    assert_eq!(i, 46);
                    break;
                }
                None => (),
            }
        }
    }
}
