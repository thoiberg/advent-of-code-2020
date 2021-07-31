use std::collections::HashSet;
use std::io::Error as ioError;
use std::iter::FromIterator;

fn main() {
    let answers = read_puzzle_input().unwrap();
    println!("Part One Solution is: {}", part_one_solution(&answers));
    println!("Part Two Solution is: {}", part_two_solution(&answers));
}

fn part_one_solution(answers: &Vec<Vec<HashSet<char>>>) -> i32 {
    answers.iter().fold(0, |acc, group_answers| {
        let unique_answers = group_answers
            .iter()
            .fold(HashSet::new(), |hs, individual_answer| {
                hs.union(individual_answer).copied().collect()
            });

        acc + (unique_answers.len() as i32)
    })
}

fn part_two_solution(answers: &Vec<Vec<HashSet<char>>>) -> i32 {
    answers.iter().fold(0, |acc, group_answers| {
        let first_answer = group_answers.first().unwrap().clone();
        let same_answers = group_answers
            .iter()
            // try and use reduce
            .fold(first_answer, |hs, individual_answer| {
                hs.intersection(individual_answer).copied().collect()
            });

        acc + (same_answers.len() as i32)
    })
}

fn read_puzzle_input() -> Result<Vec<Vec<HashSet<char>>>, ioError> {
    let contents = include_str!("puzzle_data");

    process_data(contents)
}

fn process_data(data: &str) -> Result<Vec<Vec<HashSet<char>>>, ioError> {
    let answers: Vec<Vec<HashSet<char>>> = data
        .split("\n\n")
        .map(|groups_answers| {
            groups_answers
                .split("\n")
                .map(String::from)
                .map(|s| HashSet::from_iter(s.chars()))
                .collect()
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

    #[test]
    fn part_two_solution_with_example_data() {
        let contents = include_str!("example_data");
        let answers = process_data(contents);

        assert_eq!(part_two_solution(&answers.unwrap()), 6);
    }

    #[test]
    fn part_two_solution_with_puzzle_data() {
        let contents = include_str!("puzzle_data");
        let answers = process_data(contents);

        assert_eq!(part_two_solution(&answers.unwrap()), 3323);
    }
}
