pub trait Table {
    fn table(&self);
    fn table_with_width(&self, part1_widths: &Widths, part2_widths: &Widths);
}

struct Solution {
    total_time: Option<Duration>,
    parse_time: Option<Duration>,
    solution_time: Option<Duration>,
    solution: Option<String>,
}

impl Solution {
    pub fn new() -> Self {
        Self {
            total_time: None,
            parse_time: None,
            solution_time: None,
            solution: None,
        }
    }
}

pub struct Widths {
    total_time: usize,
    parse_time: usize,
    solution_time: usize,
    solution: usize,
}

impl Widths {
    fn new(solution: &Solution) -> Self {
        let total_time = solution
            .total_time
            .map(|t| t.as_micros().to_string().len() + 3)
            .unwrap_or(0)
            .max(5);
        let parse_time = solution
            .parse_time
            .map(|t| t.as_micros().to_string().len() + 3)
            .unwrap_or(0)
            .max(5);
        let solution_time = solution
            .solution_time
            .map(|t| t.as_micros().to_string().len() + 3)
            .unwrap_or(0)
            .max(6);

        let solution = solution
            .solution
            .as_ref()
            .map(|s| s.len())
            .unwrap_or(0)
            .max(6);

        Widths {
            total_time,
            parse_time,
            solution_time,
            solution,
        }
    }
    fn get_max(widths: Vec<Self>) -> Self {
        Self {
            total_time: widths
                .iter()
                .map(|widths| widths.total_time)
                .max()
                .unwrap_or(0),
            solution_time: widths
                .iter()
                .map(|widths| widths.solution_time)
                .max()
                .unwrap_or(0),
            parse_time: widths
                .iter()
                .map(|widths| widths.parse_time)
                .max()
                .unwrap_or(0),
            solution: widths
                .iter()
                .map(|widths| widths.solution)
                .max()
                .unwrap_or(0),
        }
    }
}

impl Table for Puzzle {
    fn table(&self) {
        let part1_widths = Widths::new(&self.part1);
        let part2_widths = Widths::new(&self.part2);

        self.table_with_width(&part1_widths, &part2_widths)
    }

    fn table_with_width(&self, part1_width: &Widths, part2_width: &Widths) {
        let part1_solution = self.part1.solution.clone().unwrap_or(String::from("None"));

        let part2_solution = self.part2.solution.clone().unwrap_or(String::from("None"));

        let part1_time = self
            .part1
            .total_time
            .map(|t| t.as_micros().to_string() + " μs")
            .unwrap_or(String::from("None"));

        let part1_solution_time = self
            .part1
            .solution_time
            .map(|t| t.as_micros().to_string() + " μs")
            .unwrap_or(String::from("None"));

        let part2_time = self
            .part2
            .total_time
            .map(|t| t.as_micros().to_string() + " μs")
            .unwrap_or(String::from("None"));

        let part2_solution_time = self
            .part2
            .solution_time
            .map(|t| t.as_micros().to_string() + " μs")
            .unwrap_or(String::from("None"));

        println!(
            "┃ {0:>3} │ {1:^2$} │ {3:^4$} │ {5:^6$} │ {7:^6$} │ {8:^9$} │ {10:^9$} ┃",
            self.day,
            part1_solution,
            part1_width.solution,
            part2_solution,
            part2_width.solution,
            part1_time,
            part1_width.total_time.max(part2_width.total_time),
            part2_time,
            part1_solution_time,
            part1_width.solution_time.max(part2_width.solution_time),
            part2_solution_time,
        );
    }
}

impl Table for Year {
    fn table(&self) {
        let part1_widths = self
            .puzzles
            .iter()
            .map(|puzzle| -> Widths { Widths::new(&puzzle.part1) })
            .collect();

        let part2_widths = self
            .puzzles
            .iter()
            .map(|puzzle| -> Widths { Widths::new(&puzzle.part2) })
            .collect();

        self.table_with_width(
            &Widths::get_max(part1_widths),
            &Widths::get_max(part2_widths),
        )
    }

    fn table_with_width(&self, part1_width: &Widths, part2_width: &Widths) {
        println!(
            "┏━━━━━┳━{0:━>1$}━┳━{0:━>2$}━┳━{0:━>3$}━━━{0:━>3$}━┳━{0:━>4$}━━━{0:━>4$}━┓",
            "",
            part1_width.solution,
            part2_width.solution,
            part1_width.total_time.max(part2_width.total_time),
            part1_width.solution_time.max(part2_width.solution_time),
        );
        println!(
            "┃ Day ┃ {0:^1$} ┃ {2:^3$} ┃ {4:^5$} ┃ {6:^7$} ┃",
            "Part 1",
            part1_width.solution,
            "Part 2",
            part2_width.solution,
            "Total Time",
            2 * part1_width.total_time.max(part2_width.total_time) + 3,
            "Solution Time",
            2 * part1_width.solution_time.max(part2_width.solution_time) + 3,
        );
        println!(
            "┣━━━━━╇━{0:━>1$}━╇━{0:━>2$}━╇━{0:━>3$}━┯━{0:━>3$}━╇━{0:━>4$}━┯━{0:━>4$}━┫",
            "",
            part1_width.solution,
            part2_width.solution,
            part1_width.total_time.max(part2_width.total_time),
            part1_width.solution_time.max(part2_width.solution_time),
        );
        for puzzle in self.puzzles.iter() {
            puzzle.table_with_width(part1_width, part2_width);
        }
        println!(
            "┣━━━━━┷━{0:━>1$}━┷━{0:━>2$}━╈━{0:━>3$}━┿━{0:━>3$}━╈━{0:━>4$}━┿━{0:━>4$}━┫",
            "",
            part1_width.solution,
            part2_width.solution,
            part1_width.total_time.max(part2_width.total_time),
            part1_width.solution_time.max(part2_width.solution_time),
        );

        let total_time_part1: u128 = self
            .puzzles
            .iter()
            .map(|puzzle| puzzle.part1.total_time.map(|d| d.as_micros()).unwrap_or(0))
            .sum();
        let total_time_part2: u128 = self
            .puzzles
            .iter()
            .map(|puzzle| puzzle.part2.total_time.map(|d| d.as_micros()).unwrap_or(0))
            .sum();
        let total_time = total_time_part1 + total_time_part2;

        let solution_time_part1: u128 = self
            .puzzles
            .iter()
            .map(|puzzle| {
                puzzle
                    .part1
                    .solution_time
                    .map(|d| d.as_micros())
                    .unwrap_or(0)
            })
            .sum();
        let solution_time_part2: u128 = self
            .puzzles
            .iter()
            .map(|puzzle| {
                puzzle
                    .part2
                    .solution_time
                    .map(|d| d.as_micros())
                    .unwrap_or(0)
            })
            .sum();
        let solution_time = solution_time_part1 + solution_time_part2;

        println!(
            "┃{0:>1$}┃ {3:^2$} │ {4:^2$} ┃ {6:^5$} │ {7:^5$} ┃",
            "",
            part1_width.solution + part2_width.solution + 11,
            part1_width.total_time.max(part2_width.total_time),
            (total_time_part1 / 1000).to_string() + " ms",
            (total_time_part2 / 1000).to_string() + " ms",
            part1_width.solution_time.max(part2_width.solution_time),
            (solution_time_part1 / 1000).to_string() + " ms",
            (solution_time_part2 / 1000).to_string() + " ms",
        );
        println!(
            "┃{4:^1$}┣━{0:━^2$}━┷━{0:━^2$}━╋━{0:━>3$}━┷━{0:━^3$}━┫",
            "",
            part1_width.solution + part2_width.solution + 11,
            part1_width.total_time.max(part2_width.total_time),
            part1_width.solution_time.max(part2_width.solution_time),
            "Year: ".to_owned() + &self.year.to_string(),
        );
        println!(
            "┃{0:>1$}┃ {3:^2$} ┃ {5:^4$} ┃",
            "",
            part1_width.solution + part2_width.solution + 11,
            2 * part1_width.total_time.max(part2_width.total_time) + 3,
            (total_time / 1000).to_string() + " ms",
            2 * part1_width.solution_time.max(part2_width.solution_time) + 3,
            (solution_time / 1000).to_string() + " ms",
        );
        println!(
            "┗{0:━>1$}┻━{0:━^2$}━┻━{0:━^3$}━┛",
            "",
            part1_width.solution + part2_width.solution + 11,
            2 * part1_width.total_time.max(part2_width.total_time) + 3,
            2 * part1_width.solution_time.max(part2_width.solution_time) + 3,
        );
    }
}
