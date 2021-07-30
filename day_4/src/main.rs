use std::io::Error as ioError;

fn main() {
    let passports = read_input().unwrap();

    let first_solution = part_one_solution(passports);

    println!("The Part One Solution is: {}", first_solution);
}

fn part_one_solution(passports: Vec<Vec<String>>) -> i32 {
    // have a passport struct
    // have all the values expressed as options properties
    // add a new method that takes an array of strings and separates
    //  them into k-v pairs
    // have a match expression that the value of the property based on
    //  the key. If it exists then set it as a Some, otherwise set it to None
    let mut passport_objs: Vec<Passport> = vec![];

    for passport in passports {
        passport_objs.push(Passport::new(passport));
    }

    passport_objs.iter().fold(0, |acc, passport| {
        if passport.clone().is_valid() {
            return acc + 1;
        }
        acc
    })
}

#[derive(Default, Debug, Clone)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    pub fn new(passport_data: Vec<String>) -> Passport {
        let mut passport: Passport = Default::default();

        for data in passport_data {
            let key_value: Vec<&str> = data.split(':').collect();
            let value = Some(key_value[1].to_string());

            match key_value[0] {
                "byr" => passport.birth_year = value,
                "iyr" => passport.issue_year = value,
                "eyr" => passport.expiration_year = value,
                "hgt" => passport.height = value,
                "hcl" => passport.hair_color = value,
                "ecl" => passport.eye_color = value,
                "pid" => passport.passport_id = value,
                "cid" => passport.country_id = value,
                _ => (),
            }
        }

        passport
    }

    pub fn is_valid(self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }
}

fn read_input() -> Result<Vec<Vec<String>>, ioError> {
    let contents = include_str!("puzzle_data");
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

    #[test]
    fn test_part_one_solution() {
        assert_eq!(part_one_solution(read_input().unwrap()), 190);
    }
}
