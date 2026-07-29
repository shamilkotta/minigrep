pub fn search<'a >(pattern: &str, content: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();

  for line in content.lines() {
    if line.contains(pattern) {
        result.push(line);
    }
  }

  result
}

pub fn search_case_insensitive<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let pattern = pattern.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&pattern) {
            result.push(line);
        }
    }

    result

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
        let content ="\
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