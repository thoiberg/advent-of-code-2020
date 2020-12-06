use std::io::Error as ioError;

fn main() {
    let data = read_and_process_input().unwrap();

    let one_answer = part_one_solution(&data);

    match one_answer {
        Some(x) => println!("Part One Solution is: {}", x),
        None => println!("No answer for Part One found :("),
    }
}

fn part_one_solution(data: &Vec<i32>) -> Option<i32> {
    for i in data {
        for j in data {
            if i + j == 2020 {
                return Some(i * j);
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
