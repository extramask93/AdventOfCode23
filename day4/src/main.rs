use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr, collections::{HashSet, VecDeque},
};

use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{self, u32},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
#[derive(Clone,Debug)]
struct Card {
    id: u32,
    winningnumbers: Vec<u32>,
    scratched: Vec<u32>,
    won_numbers: Vec<u32>
}
fn card_id(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("Card")(input)?;
    let (input, number) = preceded(complete::multispace1,complete::u32)(input)?;
    Ok((input, number))
}
fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, digits) = preceded(complete::multispace0,
        separated_list1(complete::multispace1, complete::u32))(input)?;
    Ok((input, digits))
}

impl FromStr for Card {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, id) = card_id(s).unwrap();
        let (input, winningnumbers) =
            preceded(tag(":"), preceded(complete::multispace0, numbers))(input).unwrap();
        let (_, scratched) =
            preceded(preceded(complete::multispace0, tag("|")), numbers)(input).unwrap();
        let mut winningset: HashSet<u32> = HashSet::new();
        winningset.extend(winningnumbers.iter());
        let mut scratchedset: HashSet<u32> = HashSet::new();
        scratchedset.extend(scratched.iter());
        let won_numbers: Vec<u32> = winningset.intersection(&scratchedset).copied().collect();
        Ok(Card {
            id,
            winningnumbers,
            scratched,
            won_numbers
        })
    }
}
fn calc_part1(cards: &Vec<Card>) -> u32 {
    cards.iter().fold(0, |acc, card| {
        let power = card.won_numbers.iter().fold(0, |init, _| {
            if init == 0 {
                return 1;
            } else {
                init * 2
            }
        });
        acc + power
    })
}
fn calc_part2(cards: &Vec<Card>) -> u32 {
    //hold cards as references to avoid unnecessary copying
    let mut stack: Vec<&Card> = Vec::<&Card>::new();
    cards.iter().for_each(|card|
        {
            (1..=card.won_numbers.len()).for_each(|idx| {
               stack.push(&cards[(card.id as usize + idx)-1]);
            });
        });
    let mut total = cards.len() as u32;
    loop {
        match stack.pop() {
            Some(card) => {
            total+=1;
            (1..=card.won_numbers.len()).for_each(|idx| {
               stack.push(&cards[(card.id as usize + idx)-1]);
            });
            },
            None => break,
        }
    }
    total
}
fn main() {
    let f = File::open("data.txt").unwrap();
    let reader = BufReader::new(f);
    let cards: Vec<Card> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.parse::<Card>().expect("Card shoul be parseable"))
        .collect();
    println!("{}",calc_part1(&cards));
    println!("{}",calc_part2(&cards));
}

#[cfg(test)]
mod test {

    use std::collections::HashSet;

    use crate::{Card, calc_part1, calc_part2};

    #[test]
    fn parse_single_card() {
        let input = r"Card   8: 76  7 55  3 95 17 24 23 69 47 |  8 41 67 46 29 18  2 82 86 59 88 22 98 25 95 15 57 26 63  3 36 85  7 24 20";
        let card = input.parse::<Card>().expect("Card shoul be parseable");
        assert_eq!(card.id, 8);
        assert_eq!(card.winningnumbers.len(), 10);
        assert_eq!(card.scratched.len(), 25);
    }
    #[test]
    fn parse_single_card_2() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = input.parse::<Card>().expect("Card shoul be parseable");
        assert_eq!(card.id, 1);
        assert_eq!(card.winningnumbers.len(), 5);
        assert_eq!(card.scratched.len(), 8);
    }
    #[test]
    fn parse_multiple_cards() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let cards: Vec<Card> = input
            .lines()
            .map(|line| line.parse::<Card>().expect("Card shoul be parseable"))
            .collect();
        assert_eq!(13, calc_part1(&cards));
    }
    #[test]
    fn calc_second() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let cards: Vec<Card> = input
            .lines()
            .map(|line| line.parse::<Card>().expect("Card shoul be parseable"))
            .collect();
        let result = calc_part2(&cards);
        assert_eq!(result, 30);

    }

}
