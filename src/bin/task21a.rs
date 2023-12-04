use advcode_rust_template::file_reader;

fn main() {
    print!("{}", solver("data/day01/test1.txt"));
}

fn solver(path : &str) -> u32 {
    let lines : Vec<String> = file_reader::lines_from_file(path);
    0
}

#[cfg(test)]
mod tests {
    use  super::solver;

    #[test]
    fn it_works() {
        let result = solver("data/day01/test1.txt");
        assert_eq!(result, 0);
    }
}