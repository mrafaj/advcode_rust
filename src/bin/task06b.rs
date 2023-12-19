use advcode_rust_template::file_reader;

fn main() {
    print!("{}", solver("data/day06/input.txt"));
}

fn get_victory_numbers(time: &u64, distance: &u64) -> u64 {
    let time = *time as f64;
    let distance = *distance as f64;

    // The victory case can be represented with the following formula:
    // -hold_time^2 + time * hold_time - distance > 0
    // AKA quadratic formula with and always concave shape and:
    // a = -1, b = time ,c = -distance
    // solving the quadratic equation gives us a result of time/2 +- sqrt(time*time-4*distance)/2
    // victories are the number of integers between the results
    let solution_center = time / 2.0;
    let solution_offset = (time.powf(2.0) - 4.0 * distance).sqrt() / 2.0;

    let lower_bound = solution_center - solution_offset;
    let upper_bound = solution_center + solution_offset;

    let result_offset: u64 = if lower_bound.fract() == 0.0 && upper_bound.fract() == 0.0 {
        1
    } else {
        0
    };

    if solution_offset.is_nan() {
        return 0
    } else {
        return upper_bound.floor() as u64 - lower_bound.floor() as u64 - result_offset
    }
}

fn parse_line(line: &str) -> u64 {
    line.split_whitespace().skip(1).fold(String::new(), |a, b| a + b).parse::<u64>().unwrap()
}

fn solver(path : &str) -> u64 {
    let lines : Vec<String> = file_reader::lines_from_file(path);
    let time: u64 = parse_line(&lines[0]);
    let distance: u64 = parse_line(&lines[1]);
    get_victory_numbers(&time, &distance)
}

#[cfg(test)]
mod tests {
    use  super::solver;

    #[test]
    fn it_works() {
        let result = solver("data/day06/test.txt");
        assert_eq!(result, 71503);
    }
}