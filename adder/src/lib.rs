pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_positive_numbers() {
        let result = add(420, 69);
        assert_eq!(result, 489);
    }
}
