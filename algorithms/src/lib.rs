pub fn is_palindrome(s: String) -> bool {
    let mut iter = s.chars().filter(|c| c.is_alphanumeric());

    while let (Some(front), Some(back)) = (iter.next(), iter.next_back()) {
        if !front.eq_ignore_ascii_case(&back) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod is_palindrome_tests {
    use crate::is_palindrome;

    #[test]
    fn scenario_1() {
        let input = "A man, a plan, a canal: Panama";

        let is_palindrome = is_palindrome(input.to_string());

        assert!(is_palindrome);
    }

    #[test]
    fn scenario_2() {
        let input = "race a car";

        let is_palindrome = is_palindrome(input.to_string());

        assert!(!is_palindrome);
    }


    #[test]
    fn scenario_3() {
        let input = " 🎯👀 ";

        let is_palindrome = is_palindrome(input.to_string());

        assert!(is_palindrome);
    }
}