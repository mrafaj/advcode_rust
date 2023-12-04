use std::collections::HashSet;

use advcode_rust_template::file_reader;

fn main() {
    print!("{}", solver("data/day04/input.txt"));
}

#[derive(Clone, Copy)]
struct ClonedCards {
    n: u32,
    remaining_turns: u32,
}

impl ClonedCards {
    fn apply(self, total_cards: &mut u32) -> Self {
        *total_cards += &self.n;
        ClonedCards { n:self.n, remaining_turns: self.remaining_turns - 1 }
    }
}

fn parse_line(line: &String) -> (HashSet<u32>, HashSet<u32>) {
    let mut split_line = line.split(": ").nth(1).unwrap().split(" | ");
    let winning_numbers: HashSet<u32> = split_line.next().unwrap().split_whitespace().map(|n| n.parse().unwrap()).collect();
    let numbers_you_have: HashSet<u32> = split_line.next().unwrap().split_whitespace().map(|n| n.parse().unwrap()).collect();
    (winning_numbers, numbers_you_have)
}

fn get_match_number((winning_numbers, numbers_you_have): (HashSet<u32>, HashSet<u32>)) -> u32 {
    numbers_you_have.intersection(&winning_numbers).collect::<Vec<_>>().len() as u32
}

fn solver(path : &str) -> u32 {
    let lines : Vec<String> = file_reader::lines_from_file(path);
    let match_numbers = lines.iter().map(parse_line).map(get_match_number);

    let mut total_cards: u32 = 0;
    let mut card_winners: Vec<ClonedCards> = vec![];
    for match_number in match_numbers {
        let mut n_cards = 1;
        card_winners = card_winners.iter().map(|&element| element.apply(&mut n_cards)).filter(|element| element.remaining_turns > 0).collect();
        if match_number > 0 {
            card_winners.push(ClonedCards { n: n_cards, remaining_turns: match_number });
        }
        total_cards += n_cards;
    }
    total_cards
}

#[cfg(test)]
mod tests {
    use  super::solver;

    #[test]
    fn it_works() {
        let result = solver("data/day04/test.txt");
        assert_eq!(result, 30);
    }
}