use std::cmp;
use std::io::Error as ioError;

fn main() {
    let boarding_passes = read_input().unwrap();

    println!(
        "Part One Solution is: {}",
        part_one_solution(&boarding_passes)
    );
    println!(
        "Part Two Solution is: {}",
        part_two_solution(&boarding_passes)
    )
}

fn part_one_solution(boarding_passes: &[BoardingPass]) -> i32 {
    boarding_passes
        .iter()
        .fold(0, |acc, pass| cmp::max(acc, pass.seat_id()))
}

fn part_two_solution(boarding_passes: &[BoardingPass]) -> i32 {
    let all_possible_seat_range: std::ops::RangeInclusive<i32> = 0..=(127 * 8 + 7);
    let all_possible_seat_ids: Vec<i32> = all_possible_seat_range.collect();

    let known_seat_ids: Vec<i32> = boarding_passes.iter().map(|pass| pass.seat_id()).collect();

    let empty_seats = all_possible_seat_ids.into_iter().find(|seat_id| {
        !known_seat_ids.contains(seat_id)
            && known_seat_ids.contains(&(seat_id + 1))
            && known_seat_ids.contains(&(seat_id - 1))
    });

    empty_seats.unwrap()
}

fn read_input() -> Result<Vec<BoardingPass>, ioError> {
    let contents = include_str!("puzzle_data");

    process_data(contents)
}

fn process_data(data: &str) -> Result<Vec<BoardingPass>, ioError> {
    let boarding_passes: Vec<BoardingPass> = data
        .split("\n")
        .map(|s| BoardingPass::new(String::from(s)))
        .collect();
    Ok(boarding_passes)
}

struct BoardingPass {
    row_locator: Vec<char>,
    column_locator: Vec<char>,
}

impl BoardingPass {
    pub fn new(pass: String) -> BoardingPass {
        let pass_chars: Vec<char> = pass.chars().collect();

        let row_locator = pass_chars[0..=6].to_vec();
        let column_locator = pass_chars[7..].to_vec();

        BoardingPass {
            row_locator: row_locator,
            column_locator: column_locator,
        }
    }

    pub fn row(&self) -> i32 {
        let mut row_number = 0..=127;

        for direction in &self.row_locator {
            let step: f32 = (*row_number.end() as f32 - *row_number.start() as f32) / 2_f32;

            row_number = match direction {
                'F' => *row_number.start()..=(*row_number.end() - step.ceil() as i32),
                'B' => (*row_number.start() + step.ceil() as i32)..=*row_number.end(),
                _ => panic!("How did I get here?!?!?!? direction: {}", direction),
            };
        }

        *row_number.end()
    }

    pub fn column(&self) -> i32 {
        let mut column_number = 0..=7;

        for direction in &self.column_locator {
            let step: f32 = (*column_number.end() as f32 - *column_number.start() as f32) / 2_f32;

            column_number = match direction {
                'L' => *column_number.start()..=(*column_number.end() - step.ceil() as i32),
                'R' => (*column_number.start() + step.ceil() as i32)..=*column_number.end(),
                _ => panic!("How did I get here?!?!?!? direction: {}", direction),
            };
        }

        *column_number.end()
    }

    pub fn seat_id(&self) -> i32 {
        (self.row() * 8) + self.column()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_returns_seat_row() {
        assert_eq!(BoardingPass::new(String::from("FBFBBFFRLR")).row(), 44);
        assert_eq!(BoardingPass::new(String::from("BFFFBBFRRR")).row(), 70);
        assert_eq!(BoardingPass::new(String::from("FFFBBBFRRR")).row(), 14);
        assert_eq!(BoardingPass::new(String::from("BBFFBBFRLL")).row(), 102);
    }

    #[test]
    fn column_returns_seat_column() {
        assert_eq!(BoardingPass::new(String::from("FBFBBFFRLR")).column(), 5);
        assert_eq!(BoardingPass::new(String::from("BFFFBBFRRR")).column(), 7);
        assert_eq!(BoardingPass::new(String::from("FFFBBBFRRR")).column(), 7);
        assert_eq!(BoardingPass::new(String::from("BBFFBBFRLL")).column(), 4);
    }

    #[test]
    fn seat_id_returns_computed_seat_id() {
        assert_eq!(BoardingPass::new(String::from("FBFBBFFRLR")).seat_id(), 357);
        assert_eq!(BoardingPass::new(String::from("BFFFBBFRRR")).seat_id(), 567);
        assert_eq!(BoardingPass::new(String::from("FFFBBBFRRR")).seat_id(), 119);
        assert_eq!(BoardingPass::new(String::from("BBFFBBFRLL")).seat_id(), 820);
    }

    #[test]
    fn example_data_returns_max_seat_id() {
        let contents = include_str!("example_data");
        let boarding_passes = process_data(contents).unwrap();

        assert_eq!(part_one_solution(&boarding_passes), 820);
    }

    #[test]
    fn part_one_solution_is_correct() {
        let boarding_passes = read_input().unwrap();

        assert_eq!(part_one_solution(&boarding_passes), 951);
    }

    #[test]
    fn part_two_solution_is_correct() {
        let boarding_passes = read_input().unwrap();

        assert_eq!(part_two_solution(&boarding_passes), 653);
    }
}
