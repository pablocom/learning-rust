use std::fmt;

#[derive(Debug, PartialEq)]
pub struct SearchMatch<'a> {
    pub line_number: usize,
    pub line_content: &'a str,
}

impl<'a> fmt::Display for SearchMatch<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line {}: {}", self.line_number, self.line_content)
    }
}

pub fn search_case_sensitive<'a>(
    text_to_find: &str,
    file_content: &'a str,
) -> Vec<SearchMatch<'a>> {
    let mut results = Vec::new();

    for (index, line) in file_content.lines().enumerate() {
        if line.contains(text_to_find) {
            results.push(SearchMatch {
                line_number: index + 1,
                line_content: line,
            });
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    text_to_find: &str,
    file_content: &'a str,
) -> Vec<SearchMatch<'a>> {
    let text_to_lower = text_to_find.to_lowercase();
    let mut results = Vec::new();

    for (index, line) in file_content.lines().enumerate() {
        if line.to_lowercase().contains(&text_to_lower) {
            results.push(SearchMatch {
                line_number: index + 1,
                line_content: line,
            });
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

        let empty: Vec<SearchMatch> = vec![];
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
            vec![SearchMatch {
                line_number: 2,
                line_content: "safe, fast, productive."
            }],
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
            vec![
                SearchMatch {
                    line_number: 1,
                    line_content: "Rust:"
                },
                SearchMatch {
                    line_number: 2,
                    line_content: "safe, fast, productive."
                }
            ],
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
            vec![
                SearchMatch {
                    line_number: 1,
                    line_content: "Rust:"
                },
                SearchMatch {
                    line_number: 2,
                    line_content: "safe, fast, productive."
                },
                SearchMatch {
                    line_number: 3,
                    line_content: "Pick three."
                }
            ],
            search_case_insensitive(text_to_find, file_content)
        );
    }
}
