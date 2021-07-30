pub fn within_range(range: std::ops::RangeInclusive<i32>, value: &Option<i32>) -> bool {
    match value {
        Some(x) => range.contains(x),
        None => false,
    }
}

pub fn within_array<T: PartialEq>(array: Vec<T>, value: &Option<T>) -> bool {
    match value {
        Some(x) => array.contains(x),
        None => false,
    }
}

pub fn length_of(length: usize, value: &Option<String>) -> bool {
    match value {
        Some(x) => x.len() == length, // len is the byte length so this will probably fail if given non ASCII chars
        None => false,
    }
}

pub fn character_is(expected: char, actual: &Option<char>) -> bool {
    match actual {
        Some(x) => x == &expected,
        None => false,
    }
}

pub fn all_characters_are_int(value: &Option<String>) -> bool {
    match value {
        Some(string) => string.parse::<i32>().is_ok(),
        None => false,
    }
}

pub fn all_characters_are_hex(value: &Option<String>) -> bool {
    let hex_chars: Vec<char> = vec![
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    match value {
        Some(string) => string
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .all(|x| hex_chars.contains(&x)),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn within_range_returns_true_when_in_range() {
        assert_eq!(within_range(1..=10, &Some(4)), true);
    }

    #[test]
    fn within_range_returns_true_when_on_range_edge() {
        assert_eq!(within_range(1..=10, &Some(1)), true);
        assert_eq!(within_range(1..=10, &Some(10)), true);
    }

    #[test]
    fn within_range_returns_false_when_none() {
        assert_eq!(within_range(1..=10, &None), false);
    }

    #[test]
    fn within_array_returns_false_when_value_is_none() {
        let value = within_array(vec!["Apple", "Banana"], &None);
        assert_eq!(value, false);
    }

    #[test]
    fn within_array_returns_false_when_value_not_in_array() {
        let value = within_array(vec!["Apple", "Banana"], &Some("Mango"));
        assert_eq!(value, false);
    }

    #[test]
    fn within_array_returns_true_when_value_in_array() {
        let value = within_array(vec!["Apple", "Banana"], &Some("Apple"));
        assert_eq!(value, true);
    }

    #[test]
    fn length_of_returns_false_when_value_is_none() {
        assert_eq!(length_of(1, &None), false)
    }

    #[test]
    fn length_of_returns_false_when_value_length_does_not_match() {
        assert_eq!(length_of(3, &Some(String::from("a"))), false)
    }

    #[test]
    fn length_of_returns_true_when_value_length_does_match() {
        assert_eq!(length_of(3, &Some(String::from("abc"))), true)
    }

    #[test]
    fn character_is_returns_false_when_value_is_none() {
        assert_eq!(character_is('a', &None), false)
    }

    #[test]
    fn character_is_returns_false_when_value_does_not_match() {
        assert_eq!(character_is('a', &Some('b')), false)
    }

    #[test]
    fn character_is_returns_true_when_value_matches() {
        assert_eq!(character_is('a', &Some('a')), true)
    }

    #[test]
    fn all_characters_are_int_returns_false_when_value_none() {
        assert_eq!(all_characters_are_int(&None), false)
    }

    #[test]
    fn all_characters_are_int_returns_false_when_value_not_a_number() {
        assert_eq!(all_characters_are_int(&Some(String::from("apple"))), false)
    }

    #[test]
    fn all_characters_are_int_returns_true_when_value_is_number() {
        assert_eq!(
            all_characters_are_int(&Some(String::from("12345678"))),
            true
        )
    }
}
