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

pub fn by_char(input: &PuzzleInput) -> Vec<char> {
    input.0.chars().collect()
}

pub fn by_line_and_char_as_int(input: &PuzzleInput) -> Vec<Vec<i32>> {
    input
        .0
        .lines()
        .map(|line| -> Vec<i32> {
            line.chars()
                .map(|c| -> i32 { c.to_digit(10).unwrap_or(0).try_into().unwrap_or(0) })
                .collect()
        })
        .collect()
}
