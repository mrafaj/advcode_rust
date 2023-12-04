use advcode_rust_template::file_reader;

fn main() {
    print!("{}", solver("data/day03/input.txt"));
}

struct PartNumber {
    value: i32,
    row: i32,
    start_idx: i32,
    length: i32,
}

fn parse_part_number(line: &[u8], row: &usize,col: &mut usize) -> PartNumber {
    let mut part_number = PartNumber {
        value: 0,
        row: row.clone() as i32,
        start_idx: col.clone() as i32,
        length: 0,
    };

    while *col < line.len() && line[*col] >= 0x30 && line[*col] <= 0x39 {
        part_number.value *= 10;
        part_number.value += (line[*col] - 0x30) as i32;
        part_number.length += 1;
        *col+=1;
    }

    part_number
}

fn is_part_number_next_to_symbol(part_number: &PartNumber, matrix: &Vec<Vec<u8>>) -> bool {
    let mut result = false;
    'outer: for row in (part_number.row-1)..=(part_number.row+1) {
        if row == -1 || row == matrix.len() as i32 {
            continue;
        }
        for col in (part_number.start_idx - 1)..=(part_number.start_idx + part_number.length) {
            if col == -1 || col == matrix[row as usize].len() as i32 {
                continue;
            }
            let c = matrix[row as usize][col as usize].clone();
            if (c < 0x30 || c > 0x39) && c != 0x2e {
                result = true;
                break 'outer
            }
        }
    }
    result
}

fn solver(path: &str) -> i32 {
    let lines = file_reader::bytelines_from_file(path);
    let mut sum: i32 = 0;

    for row in 0..lines.len() {
        let mut col: usize = 0;
        while col < lines[row].len() {
            if lines[row][col] >= 0x30 && lines[row][col] <= 0x39
            {
                let part_number = parse_part_number(&lines[row], &row, &mut col);
                if is_part_number_next_to_symbol(&part_number, &lines) {
                    sum += part_number.value;
                }
            }
            col+=1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use  super::solver;

    #[test]
    fn it_works() {
        let result = solver("data/day03/test.txt");
        assert_eq!(result, 4361);
    }
}