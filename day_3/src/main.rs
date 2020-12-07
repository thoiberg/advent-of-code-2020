use std::io::Error as ioError;

fn main() {
    let data = read_input().unwrap();

    let first_solution = part_one_solution(&data);

    println!("The Part One Solution is: {}", first_solution);
}

fn part_one_solution(data: &Vec<String>) -> i32 {
    let multi_array: Vec<Vec<char>> = data.iter().map(|x| x.chars().collect()).collect();

    let number_of_columns = multi_array[0].len();
    let row_step = 1;
    let column_step = 3;
    let tree_character = '#';

    let mut row_position = 0;
    let mut col_position = 0;
    let mut tree_counter = 0;

    while row_position < multi_array.len() {
        let column_idx = col_position % number_of_columns;
        let current_position = multi_array[row_position][column_idx];

        if current_position == tree_character {
            tree_counter += 1;
        };
        row_position += row_step;
        col_position += column_step;
    }

    tree_counter
}

fn read_input() -> Result<Vec<String>, ioError> {
    let contents = include_str!("input_data");
    Ok(contents.split('\n').map(String::from).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let data = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect();

        assert_eq!(part_one_solution(&data), 7);
    }

    #[test]
    fn test_part_one_solution() {
        let data = read_input().unwrap();

        assert_eq!(part_one_solution(&data), 211);
    }
}
