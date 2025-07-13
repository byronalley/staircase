pub fn staircase(text: &str, reverse: bool) -> String {
    let mut output: Vec<&str> = text
        .lines()
        .map(|line| line.trim_end())
        .filter(|line| !line.chars().all(char::is_whitespace))
        .collect::<Vec<&str>>();

    if reverse {
        output.sort_by(|line1, line2| line2.len().cmp(&line1.len()))
    } else {
        output.sort_by(|line1, line2| line1.len().cmp(&line2.len()))
    }

    output.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        assert_eq!(staircase("", false), "");
    }

    #[test]
    fn test_whitespace_only_lines() {
        let input = "\n  \n\t\n";
        assert_eq!(staircase(input, false), "");
    }

    #[test]
    fn test_single_line() {
        let input = "hello";
        assert_eq!(staircase(input, false), "hello");
    }

    #[test]
    fn test_multiple_lines_sorted_by_length() {
        let input = "hello\nworld\n  buzz\nfoo\n  \n";
        assert_eq!(staircase(input, false), "foo\nhello\nworld\n  buzz");
    }

    #[test]
    fn test_lines_with_trailing_newline() {
        let input = "a\nbb\nccc\n";
        assert_eq!(staircase(input, false), "a\nbb\nccc");
    }

    #[test]
    fn test_mixed_whitespace_and_content() {
        let input = "  a  \n  ccc\t\n  bb \n\n";
        assert_eq!(staircase(input, false), "  a\n  bb\n  ccc");
    }

    #[test]
    fn test_preserves_indentation() {
        let input = "  \t- XX\n  \t- Y";
        assert_eq!(staircase(input, false), "  \t- Y\n  \t- XX");
    }

    #[test]
    fn test_reverses_sort_order() {
        let input = "a\nbb\nccc";
        assert_eq!(staircase(input, true), "ccc\nbb\na");
    }
}
