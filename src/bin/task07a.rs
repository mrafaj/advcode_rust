use std::collections::{BinaryHeap, HashSet, HashMap};

use advcode_rust_template::file_reader;

fn main() {
    print!("{}", solver("data/day07/input.txt"));
}

#[derive(Hash, PartialEq, Eq, Ord)]
struct Hand {
    cards: String,
    bid: u64
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum CardType {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jester,
    Queen,
    King,
    Ace
}

impl CardType {
    fn from_char(c: &char) -> CardType {
        match c {
            '2' => CardType::Two,
            '3' => CardType::Three,
            '4' => CardType::Four,
            '5' => CardType::Five,
            '6' => CardType::Six,
            '7' => CardType::Seven,
            '8' => CardType::Eight,
            '9' => CardType::Nine,
            'T' => CardType::Ten,
            'J' => CardType::Jester,
            'Q' => CardType::Queen,
            'K' => CardType::King,
            'A' => CardType::Ace,
            _ => CardType::Two
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Ord)]
#[repr(u8)]
enum HandType {
    HighCard(CardType),
    OnePair(CardType),
    TwoPair(CardType, CardType),
    ThreeKind(CardType),
    FullHouse(CardType, CardType),
    FourKind(CardType),
    FiveKind(CardType),
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(HandType::from_string(&self.cards).cmp(&HandType::from_string(&other.cards)))
    }
}

impl HandType {
    fn from_string(s: &str) -> HandType {
        let char_set: HashSet<char> = s.chars().collect();
        let mut result: HandType = Self::HighCard(CardType::Two);
        let mut cardNumbers: HashMap<CardType, usize>;

        for char in char_set {
            cardNumbers.insert(CardType::from_char(&char), s.matches(char).count());
        }

        for (cardType, cardNumber) in cardNumbers {
            if cardNumber == 5 {
                result = Self::FiveKind(cardType);
            } else if cardNumber == 4 {
                
            }
        }
        result
    }

    fn to_num(&self) -> u8 {
        match self {
            Self::HighCard(_) => 1,
            Self::OnePair(_) => 2,
            Self::TwoPair(_, _) => 3,
            Self::ThreeKind(_) => 4,
            Self::FullHouse(_, _) => 5,
            Self::FourKind(_) => 6,
            Self::FiveKind(_) => 7
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_value = self.to_num();
        let other_value = other.to_num();

        if self_value == other_value {
            return Some(self)
        } else {
            return Some(self_value.cmp(&other_value))
        }
    }
}

fn parse_line(line: String) -> Hand {
    let split_line: Vec<&str> = line.split_whitespace().collect();
    Hand {cards: split_line[0].to_string(), bid: split_line[1].parse::<u64>().unwrap()}
}

fn calculate_hand_rank(hand: &str) -> u32 {
    0
}

fn solver(path : &str) -> u64 {
    let lines : Vec<String> = file_reader::lines_from_file(path);
    let sorted_hands: BinaryHeap<Hand> = lines.into_iter().map(|line| parse_line(line)).collect();
    sorted_hands.into_iter().enumerate().map(|(i, hand)| i as u64 * hand.bid).sum()
}

#[cfg(test)]
mod tests {
    use  super::solver;

    #[test]
    fn it_works() {
        let result = solver("data/day07/test.txt");
        assert_eq!(result, 0);
    }
}