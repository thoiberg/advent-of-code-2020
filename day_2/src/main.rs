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
    0
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
}
