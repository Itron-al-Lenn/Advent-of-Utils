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
