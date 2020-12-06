use super::expression::*;

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
