use std::io::Error as ioError;

fn main() {
    println!("Hello, world!");
}

fn part_one_solution() -> i32 {
    0
}

fn read_input() -> Result<Vec<Vec<char>>, ioError> {
    let contents = include_str!("puzzle_data");
    // need to break on \n\n to separate the passports then
    // break on \n and whitespace to separate the data
    Ok(contents
        .split('\n')
        .map(String::from)
        .map(|x| x.chars().collect())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
}
