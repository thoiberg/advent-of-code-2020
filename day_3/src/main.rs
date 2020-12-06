use std::io::Error as ioError;

fn main() {
    let data = read_input().unwrap();

    let first_solution = part_one_solution(&data);

    println!("The Part One Solution is: {}", first_solution);
}

fn part_one_solution(data: &Vec<String>) -> i32 {
    0
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
}
