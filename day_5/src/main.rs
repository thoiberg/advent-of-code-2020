use std::io::Error as ioError;

fn main() {
    let boarding_passes = read_input().unwrap();
}

fn read_input() -> Result<Vec<BoardingPass>, ioError> {
    let contents = include_str!("puzzle_data");

    let strings: Vec<String> = contents.split("\n").map(String::from).collect();

    let mut boarding_passes: Vec<BoardingPass> = vec![];

    for string in strings {
        let boarding_pass = BoardingPass::new(string);
        boarding_passes.push(boarding_pass);
    }

    Ok(boarding_passes)
}

struct BoardingPass {
    pub row_locator: Vec<char>,
    pub column_locator: Vec<char>,
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

    pub fn row(self) -> i32 {
        let mut row_number = 0..=127;

        for direction in self.row_locator {
            let step: f32 = (*row_number.end() as f32 - *row_number.start() as f32) / 2_f32;

            row_number = match direction {
                'F' => *row_number.start()..=(*row_number.end() - step.ceil() as i32),
                'B' => (*row_number.start() + step.ceil() as i32)..=*row_number.end(),
                _ => panic!("How did I get here?!?!?!? direction: {}", direction),
            };
        }

        *row_number.end()
    }

    pub fn column(self) -> i32 {
        let mut column_number = 0..=7;

        for direction in self.row_locator {
            let step: f32 = (*column_number.end() as f32 - *column_number.start() as f32) / 2_f32;

            column_number = match direction {
                'L' => *column_number.start()..=(*column_number.end() - step.ceil() as i32),
                'R' => (*column_number.start() + step.ceil() as i32)..=*column_number.end(),
                _ => panic!("How did I get here?!?!?!? direction: {}", direction),
            };
        }

        *column_number.end()
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
}
