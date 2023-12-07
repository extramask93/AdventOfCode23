use regex::{self, Regex};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
#[derive(Debug, PartialEq)]
struct Subset {
    red: u32,
    green: u32,
    blue: u32,
}
impl Subset {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Subset { red, green, blue }
    }
}
impl FromStr for Subset {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rgb = (0, 0, 0);
        for sl in s.trim().split(",") {
            let mut result: u32 = 0;
            for digit in sl.trim().bytes() {
                if digit < 48 || digit > 57 {
                    break;
                }
                result *= 10;
                result += (digit - 48) as u32;
            }
            if sl.contains("red") {
                rgb.0 = result;
            }
            if sl.contains("green") {
                rgb.1 = result;
            }
            if sl.contains("blue") {
                rgb.2 = result;
            }
        }
        Ok(Subset::new(rgb.0, rgb.1, rgb.2))
    }
}
struct Bag {
    limits: (u32, u32, u32),
}
impl Bag {
    fn new(r: u32, g: u32, b: u32) -> Self {
        Bag { limits: (r, g, b) }
    }
    fn is_game_possible(&self, game: &Game) -> bool {
        game.subsets
            .iter()
            .filter(|subset| {
                subset.red > self.limits.0
                    || subset.green > self.limits.1
                    || subset.blue > self.limits.2
            })
            .count()
            == 0
    }
}
fn solve<R>(buff: &mut BufReader<R>) -> u32
where
    R: Read,
{
    let bag = Bag::new(12, 13, 14);
    buff.lines()
        .filter_map(|r| r.ok())
        .map(|line| line.parse::<Game>().unwrap())
        .filter(|game| bag.is_game_possible(game))
        .fold(0, |acc, game| acc + game.id)
}
#[derive(Debug)]
struct Game {
    id: u32,
    subsets: Vec<Subset>,
}
impl FromStr for Game {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_header, game_load) = s.split_at(s.find(":").unwrap());
        let re = Regex::new(r"Game\s+(\d+)").unwrap();
        let id = re.captures(game_header).unwrap()[1].parse::<u32>().unwrap();
        let v: Vec<Subset> = game_load
            .split(";")
            .map(|st| {
                let result = st.parse::<Subset>().unwrap();
                return result;
            })
            .collect();
        Ok(Game { id: id, subsets: v })
    }
}
fn main() {
    let file = File::open("data.txt").expect("Cant find the file");
    let mut reader = BufReader::new(file);
    println!("{}", solve(&mut reader))
}

mod test {
    use crate::{Bag, Game, Subset, solve};
    use std::io::BufReader;
    #[test]
    fn parse_a_subset() {
        let input1 = " 3 blue, 4 red;";
        let input2 = " 1 red, 20 green, 6 blue";
        let input3 = "2 green";
        assert_eq!(input1.parse::<Subset>().unwrap(), Subset::new(4, 0, 3)); //<Subset as FromStr>::from_str("foo")
        assert_eq!(input2.parse::<Subset>().unwrap(), Subset::new(1, 20, 6));
        assert_eq!(input3.parse::<Subset>().unwrap(), Subset::new(0, 2, 0));
    }
    #[test]
    fn parse_a_game() {
        let game_input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let game = game_input.parse::<Game>().unwrap();
        assert_eq!(game.subsets.len(), 3);
        assert_eq!(game.id, 2);
    }
    #[test]
    fn test_game_against_bag() {
        let bag = Bag::new(12, 13, 14);
        let game = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
            .parse::<Game>()
            .unwrap();
        let game2 = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            .parse::<Game>()
            .unwrap();
        assert!(bag.is_game_possible(&game));
        assert!(!bag.is_game_possible(&game2));
    }
    #[test]
    fn solve_an_example() {
        let buff = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = solve(&mut BufReader::new(buff.as_bytes()));
        assert_eq!(result, 8);
    }
}
