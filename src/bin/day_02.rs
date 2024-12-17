type Report = Vec<usize>;

const INPUT: &str = include_str!("../../resources/day_02.txt");

fn main() {
    let count_safe_reports: usize = solve(INPUT);
    println!("{}", count_safe_reports);
}

pub fn is_line_valid(line: &Report) -> bool {
    line.windows(2)
        .all(|w| 1 <= w[0].abs_diff(w[1]) && w[0].abs_diff(w[1]) <= 3)
        && (line.is_sorted() || line.iter().rev().is_sorted())
}

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|line: &Report| {
            is_line_valid(line)
                || (0..line.len()).any(|skipped| {
                let new_line: Report = line
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, val)| if idx != skipped { Some(*val) } else { None })
                    .collect::<Report>();
                is_line_valid(&new_line)
            })
        })
        .count() as usize
}