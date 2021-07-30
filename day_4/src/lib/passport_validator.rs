use super::passport::*;
use regex::Regex;

pub fn validate_passport(passport: &Passport) -> bool {
    validate_birth_year(&passport.birth_year)
        && validate_issue_year(&passport.issue_year)
        && validate_expiration_year(&passport.expiration_year)
        && validate_hair_color(&passport.hair_color)
        && validate_eye_color(&passport.eye_color)
        && validate_pid(&passport.passport_id)
        && validate_height(&passport.height)
}

pub fn within_range(range: std::ops::RangeInclusive<i32>, value: &Option<i32>) -> bool {
    match value {
        Some(x) => range.contains(x),
        None => false,
    }
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
    let re = Regex::new(r"\A(amb|blu|brn|gry|grn|hzl|oth)\z").unwrap();

    match eye_color {
        Some(ec) => re.is_match(ec),
        None => false,
    }
}

fn validate_hair_color(hair_color: &Option<String>) -> bool {
    let re = Regex::new(r"\A#[\da-f]{6}\z").unwrap();

    match hair_color {
        Some(string) => re.is_match(string),
        None => false,
    }
}

fn validate_pid(pid: &Option<String>) -> bool {
    let re = Regex::new(r"\A\d{9}\z").unwrap();
    match pid {
        Some(pid_value) => re.is_match(pid_value),
        None => false,
    }
}

fn validate_height(height: &Option<String>) -> bool {
    let re = Regex::new(r"(?P<number>\d{2,3})(?P<format>in|cm)").unwrap();

    match height {
        Some(height_value) => {
            let caps = re.captures(height_value);

            match caps {
                Some(cap_data) => match &cap_data["format"] {
                    "cm" => {
                        return within_range(
                            150..=193,
                            &String::from(&cap_data["number"]).parse::<i32>().ok(),
                        );
                    }
                    "in" => {
                        return within_range(
                            59..=76,
                            &String::from(&cap_data["number"]).parse::<i32>().ok(),
                        );
                    }
                    _ => false,
                },
                None => false,
            }
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

    #[test]
    fn validate_birth_year_checks_within_range() {
        assert_eq!(validate_birth_year(&Some(1920)), true);
        assert_eq!(validate_birth_year(&Some(1989)), true);
        assert_eq!(validate_birth_year(&Some(2002)), true);

        assert_eq!(validate_birth_year(&Some(1919)), false);
        assert_eq!(validate_birth_year(&Some(2003)), false);
    }

    #[test]
    fn validate_issue_year_checks_within_range() {
        assert_eq!(validate_issue_year(&Some(2010)), true);
        assert_eq!(validate_issue_year(&Some(2015)), true);
        assert_eq!(validate_issue_year(&Some(2020)), true);

        assert_eq!(validate_issue_year(&Some(2009)), false);
        assert_eq!(validate_issue_year(&Some(2021)), false);
    }

    #[test]
    fn validate_expiration_year_checks_within_range() {
        assert_eq!(validate_expiration_year(&Some(2020)), true);
        assert_eq!(validate_expiration_year(&Some(2025)), true);
        assert_eq!(validate_expiration_year(&Some(2030)), true);

        assert_eq!(validate_expiration_year(&Some(2019)), false);
        assert_eq!(validate_expiration_year(&Some(2031)), false);
    }

    #[test]
    fn validate_hair_color_is_hex_code() {
        assert_eq!(validate_hair_color(&Some(String::from("#123abc"))), true);

        assert_eq!(validate_hair_color(&Some(String::from("#123abz"))), false);
        assert_eq!(validate_hair_color(&Some(String::from("123abc"))), false);
    }

    #[test]
    fn validate_eye_color_is_one_of_options() {
        assert_eq!(validate_eye_color(&Some(String::from("amb"))), true);
        assert_eq!(validate_eye_color(&Some(String::from("blu"))), true);
        assert_eq!(validate_eye_color(&Some(String::from("brn"))), true);
        assert_eq!(validate_eye_color(&Some(String::from("gry"))), true);
        assert_eq!(validate_eye_color(&Some(String::from("grn"))), true);
        assert_eq!(validate_eye_color(&Some(String::from("hzl"))), true);
        assert_eq!(validate_eye_color(&Some(String::from("oth"))), true);

        assert_eq!(validate_eye_color(&Some(String::from("wat"))), false);
    }

    #[test]
    fn validate_pid_is_nine_digit_number() {
        assert_eq!(validate_pid(&Some(String::from("000000001"))), true);
        assert_eq!(validate_pid(&Some(String::from("0123456789"))), false);
    }

    #[test]
    fn validate_height_matches_format() {
        // assert_eq!(validate_height(&Some(String::from("60in"))), true);
        assert_eq!(validate_height(&Some(String::from("190cm"))), true);

        assert_eq!(validate_height(&Some(String::from("190in"))), false);
        assert_eq!(validate_height(&Some(String::from("190"))), false);
    }
}
