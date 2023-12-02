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

fn update_min_cubes(min_cubes: &mut HashMap<Color, u32>, (color, number): (Color, u32)) {
    if &number > &min_cubes[&color] {
        min_cubes.insert(color, number);
    }
}

fn parse_cube_result(pull: &str) -> (Color, u32) {
    let parts: Vec<&str> = pull.split(" ").collect();
    (Color::from_str(parts[1]).unwrap(), parts[0].parse().unwrap())
}

fn game_count_min_cubes(game: &str) -> HashMap<Color, u32> {
    let mut min_cubes = HashMap::from([(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)]);
    let pulls: Vec<(Color, u32)> = game.replace(";", ",").split(", ").map(|pull| parse_cube_result(pull)).collect();

    for pull in pulls {
        update_min_cubes(&mut min_cubes, pull);
    }

    min_cubes
}

fn parse_line(line: &str) -> u32 {
    let parts: Vec<&str> = line.split(": ").collect();

    game_count_min_cubes(parts[1]).values().product()
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
        assert_eq!(result, 2286);
    }
}