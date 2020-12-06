use std::io::Error as ioError;

fn main() {
    let data = read_input().unwrap();
    let passwords = parse_passwords(&data);

    let first_solution = part_one_solution(passwords);

    println!("The Part One Solution is: {}", first_solution);
}

fn part_one_solution(passwords: Vec<Password>) -> i32 {
    passwords.iter().fold(0, |acc, pwd| {
        if pwd.valid() {
            return acc + 1;
        }

        acc
    })

    // break the expression into two fields, count and character
    // search for all instances of the character in the password (not sure how)
    // convert the count into a range and determine if characters match
}

fn read_input() -> Result<Vec<String>, ioError> {
    let contents = include_str!("input_data");
    Ok(contents.split('\n').map(String::from).collect())
}

fn parse_passwords(data: &Vec<String>) -> Vec<Password> {
    data.iter().map(|val| Password::new(val)).collect()
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

#[derive(Debug)]
struct Password {
    password: String,
    expression: RangeExpression,
}

impl Password {
    fn new(data_entry: &String) -> Password {
        let pieces: Vec<&str> = data_entry.split(": ").collect();

        Password {
            password: String::from(pieces[1]),
            expression: RangeExpression::new(String::from(pieces[0])),
        }
    }

    fn valid(&self) -> bool {
        self.expression.valid(&self.password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let data = vec![
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc"),
        ];
        let passwords = parse_passwords(&data);

        assert_eq!(part_one_solution(passwords), 2);
    }

    #[test]
    fn test_part_one_solution() {
        let input_data = read_input().unwrap();
        let passwords = parse_passwords(&input_data);

        assert_eq!(part_one_solution(passwords), 424);
    }
}
