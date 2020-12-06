use std::io::Error as ioError;

fn main() {
    let data = read_and_process_input().unwrap();

    let first_solution = part_one_solution(&data);

    match first_solution {
        Some(x) => println!("Part One Solution is: {}", x),
        None => println!("No answer for Part One found :("),
    }

    let second_solution = part_two_solution(&data);

    match second_solution {
        Some(x) => println!("Part Two Solution is: {}", x),
        None => println!("No answer for Part One found :("),
    }
}

fn part_one_solution(data: &Vec<i32>) -> Option<i32> {
    for (i_index, i) in data.iter().enumerate() {
        for j in &data[i_index..] {
            if i + j == 2020 {
                return Some(i * j);
            }
        }
    }

    None
}

fn part_two_solution(data: &Vec<i32>) -> Option<i32> {
    for i in data {
        for j in data {
            for k in data {
                if i + j + k == 2020 {
                    return Some(i * j * k);
                }
            }
        }
    }

    None
}

fn read_and_process_input() -> Result<Vec<i32>, ioError> {
    let contents = include_str!("input_data");
    Ok(contents
        .split('\n')
        .map(|val| val.parse::<i32>().unwrap())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample() {
        let data = [1721, 979, 366, 299, 675, 1456].to_vec();
        assert_eq!(part_one_solution(&data).unwrap(), 514579);
    }

    #[test]
    fn test_part_one_solution_works() {
        let data = read_and_process_input().unwrap();
        assert_eq!(part_one_solution(&data).unwrap(), 970816);
    }

    #[test]
    fn test_part_two_sample() {
        let data = [1721, 979, 366, 299, 675, 1456].to_vec();
        assert_eq!(part_two_solution(&data).unwrap(), 241861950);
    }

    #[test]
    fn test_part_two_solution_works() {
        let data = read_and_process_input().unwrap();
        assert_eq!(part_two_solution(&data).unwrap(), 96047280);
    }
}
