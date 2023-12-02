use std::{str::FromStr, collections::HashMap};

use advcode_rust_template::file_reader;

fn main() {
    print!("{}", solver("data/day02/input.txt"));
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(input: &str) -> Result<Color, Self::Err> {
        match input {
            "red"   => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue"  => Ok(Color::Blue),
            _       => Err(()),
        }
    }
}

fn pull_is_valid((color, number): &(Color, u32)) -> bool {
    let max_cubes= HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
    number <= &max_cubes[color]
}

fn parse_cube_result(pull: &str) -> (Color, u32) {
    let parts: Vec<&str> = pull.split(" ").collect();
    (Color::from_str(parts[1]).unwrap(), parts[0].parse().unwrap())
}

fn game_is_valid(game: &str) -> bool {
    let pulls: Vec<(Color, u32)> = game.replace(";", ",").split(", ").map(|pull| parse_cube_result(pull)).collect();
    pulls.iter().all(|pull| pull_is_valid(pull))
}

fn parse_line(line: &str) -> u32 {
    let parts: Vec<&str> = line.split(": ").collect();
    let id: u32 = parts[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
    if game_is_valid(parts[1]) {
        id
    } else {
        0
    }
}

fn solver(path: &str) -> u32 {
    let lines: Vec<String> = file_reader::lines_from_file(path);
    lines.iter().map(|line| parse_line(line)).sum()
}

#[cfg(test)]
mod tests {
    use  super::*;

    #[test]
    fn it_works() {
        let result = solver("data/day02/test.txt");
        assert_eq!(result, 8);
    }
}