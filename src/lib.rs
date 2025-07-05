pub fn staircase(text: &str) -> String {
    let mut output: Vec<&str> = text
        .lines()
        .map(|line| line.trim_end())
        .filter(|line| !line.chars().all(char::is_whitespace))
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
        assert_eq!(staircase(input), "foo\nhello\nworld\n  buzz");
    }

    #[test]
    fn test_lines_with_trailing_newline() {
        let input = "a\nbb\nccc\n";
        assert_eq!(staircase(input), "a\nbb\nccc");
    }

    #[test]
    fn test_mixed_whitespace_and_content() {
        let input = "  a  \n  ccc\t\n  bb \n\n";
        assert_eq!(staircase(input), "  a\n  bb\n  ccc");
    }

    #[test]
    fn test_preserves_indentation() {
        let input = "  \t- XX\n  \t- Y";
        assert_eq!(staircase(input), "  \t- Y\n  \t- XX");
    }
}
