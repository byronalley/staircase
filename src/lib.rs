pub fn staircase(text: &str) -> String {
    let mut output: Vec<&str> = text
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .into_iter()
        .collect::<Vec<&str>>();

    output.sort_by_key(|line| line.len());

    output.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        assert_eq!(staircase(""), "");
    }

    #[test]
    fn test_whitespace_only_lines() {
        let input = "\n  \n\t\n";
        assert_eq!(staircase(input), "");
    }

    #[test]
    fn test_single_line() {
        let input = "hello";
        assert_eq!(staircase(input), "hello");
    }

    #[test]
    fn test_multiple_lines_sorted_by_length() {
        let input = "hello\nworld\n  buzz\nfoo\n  \n";
        assert_eq!(staircase(input), "foo\nbuzz\nhello\nworld");
    }

    #[test]
    fn test_lines_with_trailing_newline() {
        let input = "a\nbb\nccc\n";
        assert_eq!(staircase(input), "a\nbb\nccc");
    }

    #[test]
    fn test_mixed_whitespace_and_content() {
        let input = "  a  \n\tb\t\n c \n\n";
        assert_eq!(staircase(input), "a\nb\nc");
    }
}
