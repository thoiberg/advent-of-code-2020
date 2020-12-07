use std::io::Error as ioError;

fn main() {
    let data = read_input().unwrap();

    let first_solution = part_one_solution(&data);

    println!("The Part One Solution is: {}", first_solution);

    let second_solution = part_two_solution(&data);

    println!("The Part Two Solution is: {}", second_solution);
}

fn part_one_solution(data: &Vec<Vec<char>>) -> i64 {
    find_trees(data, 1, 3)
}

fn part_two_solution(data: &Vec<Vec<char>>) -> i64 {
    vec![
        find_trees(data, 1, 1),
        find_trees(data, 1, 3),
        find_trees(data, 1, 5),
        find_trees(data, 1, 7),
        find_trees(data, 2, 1),
    ]
    .iter()
    .fold(1, |acc, val| acc * val)
}

fn find_trees(data: &Vec<Vec<char>>, row_step: usize, column_step: usize) -> i64 {
    let number_of_columns = data[0].len();
    let tree_character = '#';

    let mut row_position = 0;
    let mut column_position = 0;
    let mut tree_counter = 0;

    while row_position < data.len() {
        let column_idx = column_position % number_of_columns;
        let current_position = data[row_position][column_idx];

        if current_position == tree_character {
            tree_counter += 1;
        };
        row_position += row_step;
        column_position += column_step;
    }

    tree_counter
}

fn read_input() -> Result<Vec<Vec<char>>, ioError> {
    let contents = include_str!("input_data");
    process_input(contents)
}

fn process_input(contents: &str) -> Result<Vec<Vec<char>>, ioError> {
    Ok(contents
        .split('\n')
        .map(String::from)
        .map(|x| x.chars().collect())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_data() -> Vec<Vec<char>> {
        let contents = include_str!("example_data");
        process_input(contents).unwrap()
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one_solution(&example_data()), 7);
    }

    #[test]
    fn test_part_one_solution() {
        let data = read_input().unwrap();

        assert_eq!(part_one_solution(&data), 211);
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(part_two_solution(&example_data()), 336);
    }

    #[test]
    fn test_part_two_solution() {
        let data = read_input().unwrap();

        assert_eq!(part_two_solution(&data), 3584591857);
    }
}
