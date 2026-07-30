pub fn search<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|v| v.contains(pattern)).collect()
}

pub fn search_case_insensitive<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();

    content
        .lines()
        .filter(|v| v.to_lowercase().contains(&pattern))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let pattern = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(pattern, content));
    }

    #[test]
    fn case_insensitive() {
        let pattern = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(pattern, content)
        );
    }
}
