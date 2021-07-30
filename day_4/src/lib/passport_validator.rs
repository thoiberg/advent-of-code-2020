use super::expression::*;
use super::passport::*;

pub fn validate_passport(passport: &Passport) -> bool {
    validate_birth_year(&passport.birth_year)
        && validate_issue_year(&passport.issue_year)
        && validate_expiration_year(&passport.expiration_year)
        && validate_hair_color(&passport.hair_color)
        && validate_eye_color(&passport.eye_color)
        && validate_pid(&passport.passport_id)
        && validate_height(&passport.height)
}

fn validate_birth_year(birth_year: &Option<i32>) -> bool {
    within_range(1920..=2002, birth_year)
}

fn validate_issue_year(issue_year: &Option<i32>) -> bool {
    within_range(2010..=2020, issue_year)
}

fn validate_expiration_year(expiration_year: &Option<i32>) -> bool {
    within_range(2020..=2030, expiration_year)
}

fn validate_eye_color(eye_color: &Option<String>) -> bool {
    within_array(
        vec![
            String::from("amb"),
            String::from("blu"),
            String::from("brn"),
            String::from("gry"),
            String::from("grn"),
            String::from("hzl"),
            String::from("oth"),
        ],
        eye_color,
    )
}

fn validate_hair_color(hair_color: &Option<String>) -> bool {
    match hair_color {
        Some(string) => {
            let char_vec: Vec<char> = string.chars().collect();
            length_of(7, &Some(string.clone()))
                && character_is('#', &Some(char_vec[0]))
                && all_characters_are_hex(&Some(string.replace("#", "")))
        }
        None => false,
    }
}

fn validate_pid(pid: &Option<String>) -> bool {
    match pid {
        Some(pid_value) => {
            length_of(9, &Some(pid_value.clone()))
                && all_characters_are_int(&Some(pid_value.clone()))
        }
        None => false,
    }
}

fn validate_height(height: &Option<String>) -> bool {
    let cm_symbol = String::from("cm");
    let in_symbol = String::from("in");
    match height {
        Some(height_value) => {
            let char_vec: Vec<char> = height_value.chars().rev().collect();
            let format: String = char_vec[0..=1].iter().rev().collect();
            let height_number: String = char_vec[2..].iter().rev().collect();
            let height_number_2 = height_number.clone();
            if within_array(vec![&in_symbol, &cm_symbol], &Some(&format)) {
                if format == cm_symbol {
                    return all_characters_are_int(&Some(height_number))
                        && within_range(150..=190, &height_number_2.parse::<i32>().ok());
                } else if format == in_symbol {
                    return all_characters_are_int(&Some(height_number))
                        && within_range(59..=76, &height_number_2.parse::<i32>().ok());
                } else {
                    return false;
                }
            }
            false
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_passport_returns_true_for_valid_passport() {
        let passport_data = vec![
            String::from("byr:1980"),
            String::from("iyr:2012"),
            String::from("eyr:2030"),
            String::from("hcl:#623a2f"),
            String::from("ecl:grn"),
            String::from("pid:087499704"),
            String::from("hgt:74in"),
        ];
        let passport = Passport::new(passport_data);

        assert_eq!(validate_passport(&passport), true);
    }

    #[test]
    fn validate_passport_returns_false_for_invalid_passport() {
        let passport_data = vec![
            String::from("eyr:1972"),
            String::from("cid:100"),
            String::from("hcl:#18171d"),
            String::from("ecl:amb"),
            String::from("hgt:170"),
            String::from("pid:186cm "),
            String::from("iyr:2018"),
            String::from("byr:1926"),
        ];
        let passport = Passport::new(passport_data);

        assert_eq!(validate_passport(&passport), false);
    }
}
