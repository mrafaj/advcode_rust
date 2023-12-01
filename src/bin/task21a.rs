use advcode_rust_template::file_reader;

fn main() {
    print!("{}", solver("data/dayNa.txt"));
}

fn solver(path : &str) -> T {
    let lines : Vec<String> = file_reader::lines_from_file(path);
}

#[cfg(test)]
mod tests {
    use  super::*;

    #[test]
    fn it_works() {
        let result = solver("data/dayNtest.txt");
        assert_eq!(result, RESULT);
    }
}