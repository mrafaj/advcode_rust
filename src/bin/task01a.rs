use advcode_rust_template::file_reader;

fn main() {
    print!("{}", solver("data/day1a.txt"));
}

fn solver(path : &str) -> u32 {
    let lines : Vec<String> = file_reader::lines_from_file(path);
    let mut result : u32 = 0;
    for line in lines.into_iter() {
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
        result += first_digit * 10 + last_digit;
    }
    result
}

#[cfg(test)]
mod tests {
    use  super::*;

    #[test]
    fn it_works() {
        let result = solver("data/day1test.txt");
        assert_eq!(result, 142);
    }
}