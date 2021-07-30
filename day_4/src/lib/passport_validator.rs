use super::expression::*;
use super::passport::*;

pub fn validate_passport(passport: &Passport) -> bool {
    within_range(1920..=2002, passport.birth_year);
    false
}
