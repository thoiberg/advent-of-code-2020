use day_4::lib::passport::*;
use day_4::lib::passport_validator::*;
use std::io::Error as ioError;

fn main() {
    let passports = read_input().unwrap();
    let also_passports = read_input().unwrap();

    let first_solution = part_one_solution(passports);
    let second_solution = part_two_solution(also_passports);

    println!("The Part One Solution is: {}", first_solution);
    println!("The Part Two Solution is: {}", second_solution);
}

fn part_one_solution(passports: Vec<Vec<String>>) -> i32 {
    let mut passport_objs: Vec<Passport> = vec![];

    for passport in passports {
        passport_objs.push(Passport::new(passport));
    }

    passport_objs.iter().fold(0, |acc, passport| {
        if passport.clone().is_valid() {
            return acc + 1;
        }
        acc
    })
}

fn part_two_solution(passports: Vec<Vec<String>>) -> i32 {
    let mut passport_objs: Vec<Passport> = vec![];

    for passport in passports {
        passport_objs.push(Passport::new(passport.to_vec()));
    }

    passport_objs.iter().fold(0, |acc, passport| {
        if passport_is_valid(passport) {
            return acc + 1;
        }
        acc
    })
}

fn passport_is_valid(passport: &Passport) -> bool {
    validate_passport(passport)
}

fn read_input() -> Result<Vec<Vec<String>>, ioError> {
    let contents = include_str!("puzzle_data");
    process_input(contents)
}

fn process_input(contents: &str) -> Result<Vec<Vec<String>>, ioError> {
    Ok(contents
        .split("\n\n")
        .map(|passport| {
            passport
                .split(['\n', ' '].as_ref())
                .map(String::from)
                .collect()
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Vec<String>> {
        let contents = include_str!("example_data");

        process_input(contents).unwrap()
    }

    fn part_two_test_data() -> Vec<Vec<String>> {
        let contents = include_str!("part_two_example_data");

        process_input(contents).unwrap()
    }

    #[test]
    fn test_part_one_example_data() {
        let example_data = test_data();

        assert_eq!(part_one_solution(example_data), 2);
    }

    #[test]
    fn test_part_one_solution() {
        assert_eq!(part_one_solution(read_input().unwrap()), 190);
    }

    #[test]
    fn test_part_two_valid_passports() {
        let example_data = part_two_test_data();

        assert_eq!(part_two_solution(example_data), 4);
    }

    #[test]
    fn test_part_two_solution() {
        assert_eq!(part_two_solution(read_input().unwrap()), 121)
    }
}
