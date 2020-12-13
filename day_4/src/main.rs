use std::io::Error as ioError;

fn main() {
    println!("Hello, world!");

    let passports = read_input().unwrap();

    let first_solution = part_one_solution(passports);

    println!("The Part One Solution is: {}", first_solution);
}

fn part_one_solution(passports: Vec<Vec<String>>) -> i32 {
    0
}

fn read_input() -> Result<Vec<Vec<String>>, ioError> {
    let contents = include_str!("puzzle_data");
    // need to break on \n\n to separate the passports then
    // break on \n and whitespace to separate the data
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

    #[test]
    fn test_part_one_example_data() {
        let example_data = test_data();

        assert_eq!(part_one_solution(example_data), 2);
    }
}
