// use day_2::lib::expression::*;
// use day_2::lib::password::*;
use day_2::lib::*;
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
