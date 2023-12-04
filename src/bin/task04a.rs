use std::collections::HashSet;

use advcode_rust_template::file_reader;

fn main() {
    print!("{}", solver("data/day04/input.txt"));
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

fn match_numbers_to_score(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        2_u32.pow(n - 1)
    }
}

fn solver(path : &str) -> u32 {
    let lines : Vec<String> = file_reader::lines_from_file(path);
    lines.iter().map(parse_line).map(get_match_number).map(match_numbers_to_score).sum()
}

#[cfg(test)]
mod tests {
    use  super::solver;

    #[test]
    fn it_works() {
        let result = solver("data/day04/test.txt");
        assert_eq!(result, 13);
    }
}