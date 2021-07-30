pub fn within_range(range: std::ops::RangeInclusive<i32>, value: Option<i32>) -> bool {
    if value.is_none() {
        false
    } else {
        range.contains(&value.unwrap())
    }
}
