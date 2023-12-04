use advcode_rust_template::file_reader;

fn main() {
    print!("{}", solver("data/day01/input.txt"));
}

fn parse_line(line : &str) -> u32 {
    let digits: [(&str, &str); 10] = [
        ("zero", "z0o"),
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];
    let mut line_local : String = line.to_string();
    for (text, number) in digits {
        line_local = line_local.replace(text, number);
    }

    let mut first_digit : u32 = 10;
    let mut last_digit : u32 = 10;
    for char in line_local.chars() {
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
    lines.iter().map(|line| parse_line(line)).sum()
}

#[cfg(test)]
mod tests {
    use  super::solver;

    #[test]
    fn it_works() {
        let result = solver("data/day01/test2.txt");
        assert_eq!(result, 281);
    }
}