use advent_of_utils::types::PuzzleInput;

pub fn by_line(input: &PuzzleInput) -> Vec<String> {
    input.input().lines().map(String::from).collect()
}

pub fn by_line_and_empty_line(input: &PuzzleInput) -> Vec<Vec<String>> {
    input
        .input()
        .split("\n\n")
        .map(|lines| lines.lines().map(String::from).collect())
        .collect()
}

pub fn by_char(input: &PuzzleInput) -> Vec<char> {
    input.input().chars().collect()
}

pub fn by_line_and_char_as_int(input: &PuzzleInput) -> Vec<Vec<i32>> {
    input
        .input()
        .lines()
        .map(|line| -> Vec<i32> {
            line.chars()
                .filter_map(|c| c.to_digit(10).and_then(|u| u.try_into().ok()))
                .collect()
        })
        .collect()
}

pub fn by_line_and_word_as_int(input: &PuzzleInput) -> Vec<Vec<i32>> {
    input
        .input
        .lines()
        .map(|line| -> Vec<i32> {
            line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        })
        .collect()
}
