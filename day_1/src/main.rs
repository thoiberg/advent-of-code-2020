use std::io::Error as ioError;

fn main() {
    let data = read_and_process_input().unwrap();
    println!("{}", data.len());
    println!("{}", data[0]);

    // iterate through data
    //  for each value iterate through the array again
    // add both values together. If they match 2020 multiply them together
    // and return result
    for i in &data {
        for j in &data {
            if i + j == 2020 {
                println!("Answer is: {}", i * j);
            }
        }
    }
}

fn read_and_process_input() -> Result<Vec<i32>, ioError> {
    let contents = include_str!("input_data");
    Ok(contents
        .split('\n')
        .map(|val| val.parse::<i32>().unwrap())
        .collect())
}
