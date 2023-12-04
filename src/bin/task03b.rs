use std::ops;

use advcode_rust_template::file_reader;

fn main() {
    print!("{}", solver("data/day03/input.txt"));
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Vec2 {
    row: i32,
    col: i32,
}

struct Gear {
    position: Vec2,
    adjacent_numbers: Vec<PartNumber>
}

impl PartialEq for &Gear {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}
impl ops::AddAssign for Gear {
    fn add_assign(&mut self, rhs: Self) {
        self.adjacent_numbers.append(&mut rhs.adjacent_numbers.clone())
    }
}

#[derive(Clone, Copy)]
struct PartNumber {
    value: i32,
    start: Vec2,
    length: i32,
}

fn parse_part_number(line: &[u8], row: &usize,col: &mut usize) -> PartNumber {
    let mut part_number = PartNumber {
        value: 0,
        start: Vec2 {
            row: row.clone() as i32,
            col: col.clone() as i32,
        },
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

fn find_part_number_gear(part_number: PartNumber, matrix: &Vec<Vec<u8>>, gears: &mut Vec<Gear>) {
    for row in (part_number.start.row-1)..=(part_number.start.row+1) {
        if row == -1 || row == matrix.len() as i32 {
            continue;
        }
        for col in (part_number.start.col - 1)..=(part_number.start.col + part_number.length) {
            if col == -1 || col == matrix[row as usize].len() as i32 {
                continue;
            }
            let c = matrix[row as usize][col as usize].clone();
            if c == 0x2a {
                let gear = Gear {
                    position: Vec2 {
                        row: row,
                        col: col,
                    },
                    adjacent_numbers: vec![part_number],
                };
                let gear_idx = gears.iter().position(|g| g == &gear);
                match  gear_idx {
                    Some(idx) => gears[idx] += gear,
                    None => gears.push(gear),
                }
            }
        }
    }
}

fn solver(path: &str) -> i32 {
    let lines = file_reader::bytelines_from_file(path);
    let mut gears: Vec<Gear> = vec![];

    for row in 0..lines.len() {
        let mut col: usize = 0;
        while col < lines[row].len() {
            if lines[row][col] >= 0x30 && lines[row][col] <= 0x39
            {
                let part_number = parse_part_number(&lines[row], &row, &mut col);
                find_part_number_gear(part_number, &lines, &mut gears);
            }
            col+=1;
        }
    }
    gears.iter().filter(|&gear| gear.adjacent_numbers.len() == 2).map(|gear| gear.adjacent_numbers[0].value * gear.adjacent_numbers[1].value).sum()
}

#[cfg(test)]
mod tests {
    use  super::solver;

    #[test]
    fn it_works() {
        let result = solver("data/day03/test.txt");
        assert_eq!(result, 467835);
    }
}