use std::io::Error as ioError;

fn main() {
    println!("Hello, world!");

    let data = parse_input().unwrap();

    let passwords: Vec<Password> = data
        .iter()
        .map(|val| {
            let pieces: Vec<&str> = val.split(": ").collect();
            Password {
                password: String::from(pieces[1]),
                validation_expression: String::from(pieces[0]),
            }
        })
        .collect();

    let first_solution = part_one_solution(passwords);

    println!("The Part One Solution is: {}", first_solution);
}

fn part_one_solution(passwords: Vec<Password>) -> i32 {
    passwords.iter().fold(0, |acc, pwd| {
        if password_meets_requirements(&pwd) {
            return acc + 1;
        }

        acc
    })

    // break the expression into two fields, count and character
    // search for all instances of the character in the password (not sure how)
    // convert the count into a range and determine if characters match
}

fn password_meets_requirements(password: &Password) -> bool {
    let expr: Vec<&str> = password.validation_expression.split(" ").collect();
    let count = expr[0];
    let character = expr[1];

    let password_chars: Vec<char> = password.password.chars().collect();
    let matching_count = password_chars.iter().fold(0, |acc, &char| {
        if char.to_string() == character {
            return acc + 1;
        }

        acc
    });

    let count_min_and_max: Vec<i32> = count
        .split("-")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let count_range = count_min_and_max[0]..=count_min_and_max[1];

    count_range.contains(&matching_count)
}

fn parse_input() -> Result<Vec<String>, ioError> {
    let contents = include_str!("input_data");
    Ok(contents.split('\n').map(String::from).collect())
}

#[derive(Debug)]
struct Password {
    password: String,
    validation_expression: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let passwords = vec![
            Password {
                password: String::from("abcde"),
                validation_expression: String::from("1-3 a"),
            },
            Password {
                password: String::from("cdefg"),
                validation_expression: String::from("1-3 b"),
            },
            Password {
                password: String::from("ccccccccc"),
                validation_expression: String::from("2-9 c"),
            },
        ];

        assert_eq!(part_one_solution(passwords), 2);
    }

    #[test]
    fn test_part_one_solution() {
        let input_data = parse_input().unwrap();
        let passwords: Vec<Password> = input_data
            .iter()
            .map(|val| {
                let pieces: Vec<&str> = val.split(": ").collect();
                Password {
                    password: String::from(pieces[1]),
                    validation_expression: String::from(pieces[0]),
                }
            })
            .collect();

        assert_eq!(part_one_solution(passwords), 424);
    }
}
