use crate::input::PuzzleInput;

pub fn by_line(input: &PuzzleInput) -> Vec<String> {
    input
        .0
        .split("\n")
        .map(|x| -> String { String::from(x) })
        .collect()
}
