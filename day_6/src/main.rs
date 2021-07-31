use std::collections::HashSet;
use std::io::Error as ioError;
use std::iter::FromIterator;

fn main() {
    println!("Hello, world!");

    let answers = read_puzzle_input().unwrap();
    println!("Part One Solution is: {}", part_one_solution(&answers));
}

fn part_one_solution(answers: &Vec<Vec<char>>) -> i32 {
    answers.iter().fold(0, |acc, group_answers| {
        let hash_set: HashSet<&char> = HashSet::from_iter(group_answers);

        acc + (hash_set.len() as i32)
    })
}

fn read_puzzle_input() -> Result<Vec<Vec<char>>, ioError> {
    let contents = include_str!("puzzle_data");

    process_data(contents)
}

fn process_data(data: &str) -> Result<Vec<Vec<char>>, ioError> {
    let answers: Vec<Vec<char>> = data
        .split("\n\n")
        .map(|groups_answers| {
            groups_answers
                .split("\n")
                .map(|s| {
                    // TODO: See what I can do with the Split struct.
                    // If I can remove the String building that'd be ideal
                    String::from(s);
                    s.chars()
                })
                .fold(vec![], |mut acc, chars| {
                    acc.extend(chars);
                    acc
                })
        })
        .collect();
    Ok(answers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_solution_with_example_data() {
        let contents = include_str!("example_data");
        let answers = process_data(contents);

        assert_eq!(part_one_solution(&answers.unwrap()), 11);
    }

    #[test]
    fn part_one_solution_with_puzzle_data() {
        let contents = include_str!("puzzle_data");
        let answers = process_data(contents);

        assert_eq!(part_one_solution(&answers.unwrap()), 6530);
    }
}
