pub fn search_case_sensitive<'a>(text_to_find: &str, file_content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in file_content.lines() {
        if line.contains(text_to_find) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(text_to_find: &str, file_content: &'a str) -> Vec<&'a str> {
    let text_to_lower = text_to_find.to_lowercase();
    let mut results = Vec::new();

    for line in file_content.lines() {
        if line.to_lowercase().contains(&text_to_lower) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_no_findings() {
        let text_to_find = "Safe";
        let file_content = "\
Rust:
safe, fast, productive.
Pick three.";

        let empty: Vec<&str> = vec![];
        assert_eq!(empty, search_case_sensitive(text_to_find, file_content));
    }

    #[test]
    fn case_sensitive_single_finding() {
        let text_to_find = "p";
        let file_content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(text_to_find, file_content)
        );
    }

    #[test]
    fn case_sensitive_multiple_findings() {
        let text_to_find = "st";
        let file_content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["Rust:", "safe, fast, productive."],
            search_case_sensitive(text_to_find, file_content)
        );
    }

    #[test]
    fn case_insensitive_multiple_findings() {
        let text_to_find = "R";
        let file_content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["Rust:", "safe, fast, productive.", "Pick three."],
            search_case_insensitive(text_to_find, file_content)
        );
    }
}
