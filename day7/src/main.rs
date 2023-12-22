use indexmap::IndexMap;
use nom::{
    character::complete::{self, anychar, space1, newline, line_ending},
    error::ParseError,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Eq, PartialEq, Hash, PartialOrd, Debug, Ord, Clone, Copy)]
enum Card {
    Value(u64),
    Jack,
    Queen,
    King,
    As,
}
fn parse_card(input: &str) -> IResult<&str, Card> {
    let (newinput, c) = anychar(input)?;
    match Card::try_from(c) {
        Ok(card) => Ok((newinput, card)),
        Err(_) => Err(nom::Err::Error(nom::error::Error::from_char(input, c))),
    }
}
impl TryFrom<char> for Card {
    type Error = std::io::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::As),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Value(10)),
            '9' => Ok(Card::Value(9)),
            '8' => Ok(Card::Value(8)),
            '7' => Ok(Card::Value(7)),
            '6' => Ok(Card::Value(6)),
            '5' => Ok(Card::Value(5)),
            '4' => Ok(Card::Value(4)),
            '3' => Ok(Card::Value(3)),
            '2' => Ok(Card::Value(2)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Found unknown card",
            )),
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
    hand_type: HandType,
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type == other.hand_type {
            for (left, right) in self.cards.iter().zip(other.cards.iter()) {
                if *left != *right {
                    return false;
                }
            }
            return true;
        }

        false
    }
}
impl Eq for Hand {}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            for (left, right) in self.cards.iter().zip(other.cards.iter()) {
                if *left == *right {
                    continue;
                } else {
                    return left.cmp(right);
                }
            }
            return std::cmp::Ordering::Equal;
        }
        self.hand_type.cmp(&other.hand_type)
    }
}
impl Hand {
    fn new(cards: Vec<Card>, bid: u64) -> Self {
        let hand_type = Hand::hand_type(&cards);
        Hand {
            cards,
            bid,
            hand_type,
        }
    }
    fn hand_type(cards: &[Card]) -> HandType {
        let mut cnt_map: IndexMap<Card, u8> = IndexMap::new();
        cnt_map = cards.iter().fold(cnt_map, |mut map, elem| {
            let entry = map.entry(*elem).or_insert(0);
            *entry += 1;
            map
        });
        cnt_map.sort_by(|_, vala, _, valb| valb.cmp(vala));
        let hand_type = match cnt_map.iter().next().unwrap() {
            (_, 5) => HandType::FiveOfAKind,
            (_, 4) => HandType::FourOfAKind,
            (_, 3) => {
                if *cnt_map.iter().nth(1).unwrap().1 == 1 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            (_, 2) =>{
                if *cnt_map.iter().nth(1).unwrap().1 == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            (_, 1) => HandType::HighCard,
            _ => panic!("Unknown hand type"),
        };
        hand_type
    }
}
fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, (cards, bid)) =
        separated_pair(many1(parse_card), space1, complete::u64)(input)?;
    Ok((input, Hand::new(cards, bid)))
}

fn main() {
    let input = std::fs::read_to_string("data.txt").unwrap();

    let (_, mut hands) = separated_list1(line_ending, parse_hand)(&input).unwrap();
    hands.sort();
    let total_winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (nr, hand)| acc + (hand.bid as usize * (nr + 1)));
    println!("{}", total_winnings);
}
#[cfg(test)]
mod test {
    use nom::{character::complete::newline, multi::separated_list1};

    use crate::{parse_hand, Hand, HandType};

    #[test]
    fn load_hand1() {
        let input = r"32T3K 765";
        let (_, hand) = parse_hand(input).unwrap();
        assert_eq!(hand.bid, 765);
        assert_eq!(hand.cards.len(), 5);
        assert_eq!(hand.hand_type, HandType::OnePair);
    }
    #[test]
    fn check_total_winnings() {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let (_, mut hands) = separated_list1(newline, parse_hand)(input).unwrap();
        hands.sort();
        let total_winnings = hands
            .iter()
            .enumerate()
            .fold(0, |acc, (nr, hand)| acc + (hand.bid as usize * (nr + 1)));
        assert_eq!(total_winnings, 6440);
    }
}
