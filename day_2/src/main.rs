use std::io::Error as ioError;

fn main() {
    let data = read_input().unwrap();
    let part_one_passwords = parse_passwords(&data, Expression::RangeExpression);

    let first_solution = part_one_solution(part_one_passwords);

    println!("The Part One Solution is: {}", first_solution);

    let part_two_passwords = parse_passwords(&data, Expression::PositionExpression);
    let second_solution = part_two_solution(part_two_passwords);

    println!("The Part Two Solution is: {}", second_solution);
}

fn part_one_solution(passwords: Vec<Password>) -> i32 {
    passwords.iter().fold(0, |acc, pwd| {
        if pwd.valid() {
            return acc + 1;
        }

        acc
    })
}

fn part_two_solution(passwords: Vec<Password>) -> i32 {
    passwords.iter().fold(0, |acc, pwd| {
        if pwd.valid() {
            return acc + 1;
        }

        acc
    })
}

fn read_input() -> Result<Vec<String>, ioError> {
    let contents = include_str!("input_data");
    Ok(contents.split('\n').map(String::from).collect())
}

fn parse_passwords(data: &Vec<String>, expression: Expression) -> Vec<Password> {
    data.iter()
        .map(|val| Password::new(val, expression))
        .collect()
}

trait ValidationExpression {
    fn valid(&self, password: &String) -> bool;
}

#[derive(Copy, Clone)]
enum Expression {
    RangeExpression,
    PositionExpression,
}

#[derive(Debug)]
struct RangeExpression {
    character: String,
    range: std::ops::RangeInclusive<i32>,
}

impl RangeExpression {
    fn new(range_string: String) -> RangeExpression {
        let expr: Vec<&str> = range_string.split(" ").collect();
        let range = expr[0];
        let character = expr[1];
        let range_nums: Vec<i32> = range
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        RangeExpression {
            character: String::from(character),
            range: range_nums[0]..=range_nums[1],
        }
    }
}

impl ValidationExpression for RangeExpression {
    fn valid(&self, password: &String) -> bool {
        let password_chars: Vec<char> = password.chars().collect();
        let matching_count = password_chars.iter().fold(0, |acc, &char| {
            if char.to_string() == self.character {
                return acc + 1;
            }

            acc
        });

        self.range.contains(&matching_count)
    }
}

struct PositionExpression {
    positions: Vec<i32>,
    character: String,
}

impl PositionExpression {
    fn new(range_string: String) -> PositionExpression {
        let expr: Vec<&str> = range_string.split(" ").collect();
        let positions = expr[0];
        let character = expr[1];
        let positions: Vec<i32> = positions
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        PositionExpression {
            character: String::from(character),
            positions,
        }
    }
}

impl ValidationExpression for PositionExpression {
    fn valid(&self, password: &String) -> bool {
        let password_chars: Vec<char> = password.chars().collect();
        let matching_positions = self.positions.iter().map(|pos| {
            let char_at = password_chars[(*pos - 1) as usize];
            String::from(char_at) == self.character
        });

        let number_of_trues = matching_positions.fold(0, |acc, pos| {
            if pos {
                return acc + 1;
            }

            acc
        });

        number_of_trues == 1
    }
}

struct Password {
    password: String,
    expression: Box<dyn ValidationExpression>,
}

impl Password {
    fn new(data_entry: &String, expression: Expression) -> Password {
        let pieces: Vec<&str> = data_entry.split(": ").collect();
        let expression: Box<dyn ValidationExpression> = match expression {
            Expression::RangeExpression => Box::new(RangeExpression::new(String::from(pieces[0]))),
            Expression::PositionExpression => {
                Box::new(PositionExpression::new(String::from(pieces[0])))
            }
        };

        Password {
            password: String::from(pieces[1]),
            expression,
        }
    }

    fn valid(&self) -> bool {
        self.expression.valid(&self.password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<String> {
        vec![
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc"),
        ]
    }

    #[test]
    fn test_part_one_example() {
        let passwords = parse_passwords(&test_data(), Expression::RangeExpression);

        assert_eq!(part_one_solution(passwords), 2);
    }

    #[test]
    fn test_part_one_solution() {
        let data = read_input().unwrap();
        let passwords = parse_passwords(&data, Expression::RangeExpression);

        assert_eq!(part_one_solution(passwords), 424);
    }

    #[test]
    fn test_part_two_example() {
        let passwords = parse_passwords(&test_data(), Expression::PositionExpression);

        assert_eq!(part_two_solution(passwords), 1);
    }

    #[test]
    fn test_part_two_solution() {
        let data = read_input().unwrap();
        let passwords = parse_passwords(&data, Expression::PositionExpression);

        assert_eq!(part_two_solution(passwords), 747);
    }
}
