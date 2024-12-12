pub fn by_line(input: String) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn by_line_and_empty_line(input: String) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .map(|lines| lines.lines().map(String::from).collect())
        .collect()
}

pub fn by_char(input: String) -> Vec<char> {
    input.chars().collect()
}

pub fn by_line_and_char_as_int(input: String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| -> Vec<i32> {
            line.chars()
                .filter_map(|c| c.to_digit(10).and_then(|u| u.try_into().ok()))
                .collect()
        })
        .collect()
}

pub fn by_line_and_word_as_int(input: String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| -> Vec<i32> {
            line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        })
        .collect()
}
