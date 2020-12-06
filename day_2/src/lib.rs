pub struct Password {
    password: String,
    expression: Box<dyn ValidationExpression>,
}

impl Password {
    pub fn new(data_entry: &String, expression: Expression) -> Password {
        let pieces: Vec<&str> = data_entry.split(": ").collect();
        let expression: Box<dyn ValidationExpression> = match expression {
            Expression::RangeExpression => Box::new(RangeExpression::new(String::from(pieces[0]))),
            Expression::PositionExpression => {
                Box::new(PositionExpression::new(String::from(pieces[0])))
            }
        };

        Password {
            password: String::from(pieces[1]),
            expression,
        }
    }

    pub fn valid(&self) -> bool {
        self.expression.valid(&self.password)
    }
}

trait ValidationExpression {
    fn valid(&self, password: &String) -> bool;
}

#[derive(Copy, Clone)]
pub enum Expression {
    RangeExpression,
    PositionExpression,
}

#[derive(Debug)]
struct RangeExpression {
    character: String,
    range: std::ops::RangeInclusive<i32>,
}

impl RangeExpression {
    fn new(range_string: String) -> RangeExpression {
        let expr: Vec<&str> = range_string.split(" ").collect();
        let range = expr[0];
        let character = expr[1];
        let range_nums: Vec<i32> = range
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        RangeExpression {
            character: String::from(character),
            range: range_nums[0]..=range_nums[1],
        }
    }
}

impl ValidationExpression for RangeExpression {
    fn valid(&self, password: &String) -> bool {
        let password_chars: Vec<char> = password.chars().collect();
        let matching_count = password_chars.iter().fold(0, |acc, &char| {
            if char.to_string() == self.character {
                return acc + 1;
            }

            acc
        });

        self.range.contains(&matching_count)
    }
}

struct PositionExpression {
    positions: Vec<i32>,
    character: String,
}

impl PositionExpression {
    fn new(range_string: String) -> PositionExpression {
        let expr: Vec<&str> = range_string.split(" ").collect();
        let positions = expr[0];
        let character = expr[1];
        let positions: Vec<i32> = positions
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        PositionExpression {
            character: String::from(character),
            positions,
        }
    }
}

impl ValidationExpression for PositionExpression {
    fn valid(&self, password: &String) -> bool {
        let password_chars: Vec<char> = password.chars().collect();
        let matching_positions = self.positions.iter().map(|pos| {
            let char_at = password_chars[(*pos - 1) as usize];
            String::from(char_at) == self.character
        });

        let number_of_trues = matching_positions.fold(0, |acc, pos| {
            if pos {
                return acc + 1;
            }

            acc
        });

        number_of_trues == 1
    }
}
