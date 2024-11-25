use crate::input::PuzzleInput;

pub fn by_line(input: &PuzzleInput) -> Vec<String> {
    input.0.lines().map(String::from).collect()
}

pub fn by_line_and_empty_line(input: &PuzzleInput) -> Vec<Vec<String>> {
    input
        .0
        .split("\n\n")
        .map(|lines| lines.lines().map(String::from).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::PuzzleInput;

    #[test]
    fn test_by_line() {
        let input = PuzzleInput("line1\nline2\nline3".to_string());
        let expected = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ];
        assert_eq!(by_line(&input), expected);
    }

    #[test]
    fn test_by_line_and_empty_line() {
        let input = PuzzleInput("line1\nline2\n\nline3\nline4".to_string());
        let expected = vec![
            vec!["line1".to_string(), "line2".to_string()],
            vec!["line3".to_string(), "line4".to_string()],
        ];
        assert_eq!(by_line_and_empty_line(&input), expected);
    }
}
