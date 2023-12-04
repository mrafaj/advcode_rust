use advcode_rust_template::file_reader;

fn main() {
    print!("{}", solver("data/day01/input.txt"));
}

fn parse_line(line : &str) -> u32 {
    let mut first_digit : u32 = 10;
    let mut last_digit : u32 = 10;
    for char in line.chars() {
        if char.is_digit(10) {
            if first_digit == 10 {
                first_digit = char.to_digit(10).unwrap();
            }
            last_digit = char.to_digit(10).unwrap();
        }
    }
    first_digit * 10 + last_digit
}

fn solver(path : &str) -> u32 {
    let lines : Vec<String> = file_reader::lines_from_file(path);
    lines.iter().map(|line| parse_line(&line)).sum()
}

#[cfg(test)]
mod tests {
    use  super::solver;

    #[test]
    fn it_works() {
        let result = solver("data/day01/test1.txt");
        assert_eq!(result, 142);
    }
}