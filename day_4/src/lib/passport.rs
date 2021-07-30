#[derive(Default, Debug, Clone)]
pub struct Passport {
    pub birth_year: Option<i32>,
    pub issue_year: Option<i32>,
    pub expiration_year: Option<i32>,
    pub height: Option<String>,
    pub hair_color: Option<String>,
    pub eye_color: Option<String>,
    pub passport_id: Option<String>,
    pub country_id: Option<i32>,
}

impl Passport {
    pub fn new(passport_data: Vec<String>) -> Passport {
        let mut passport: Passport = Default::default();

        for data in passport_data {
            let key_value: Vec<&str> = data.split(':').collect();
            let value = key_value[1];

            match key_value[0] {
                "byr" => passport.birth_year = value.parse().ok(),
                "iyr" => passport.issue_year = value.parse().ok(),
                "eyr" => passport.expiration_year = value.parse().ok(),
                "hgt" => passport.height = Some(value.to_string()),
                "hcl" => passport.hair_color = Some(value.to_string()),
                "ecl" => passport.eye_color = Some(value.to_string()),
                "pid" => passport.passport_id = value.parse().ok(),
                "cid" => passport.country_id = value.parse().ok(),
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
