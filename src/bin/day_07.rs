const INPUT: &str = include_str!("../../resources/day_07.txt");

fn main() {
    solve_pt1(INPUT);
    solve_pt2(INPUT);
}

fn solve_pt1(input: &str) {
    let equations: Vec<(usize, Vec<usize>)> = read_input(input);
    let sum: usize = equations
        .iter()
        .map(|(result, numbers)| -> usize {
            let (start, numbers) = numbers.split_first().unwrap();
            if is_possible_pt1(*start, *result, numbers) {
                *result
            } else {
                0
            }
        })
        .sum();

    println!("{}", sum)
}

fn solve_pt2(input: &str) {
    let equations: Vec<(usize, Vec<usize>)> = read_input(input);
    let sum: usize = equations
        .iter()
        .map(|(result, numbers)| -> usize {
            let (start, numbers) = numbers.split_first().unwrap();
            if is_possible_pt2(*start, *result, numbers) {
                *result
            } else {
                0
            }
        })
        .sum();

    println!("{}", sum)
}

fn read_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line: &str| {
            let (result, numbers): (&str, &str) = line.split_once(':').unwrap();
            let result: usize = result.parse().unwrap();
            let numbers: Vec<usize> = numbers
                .split_whitespace()
                .map(|x: &str| x.parse().unwrap())
                .collect();
            (result, numbers)
        })
        .collect()
}

fn is_possible_pt1(start: usize, target: usize, numbers: &[usize]) -> bool {
    if start > target {
        return false;
    }

    if numbers.is_empty() {
        return start == target;
    }

    let (first, rest): (&usize, &[usize]) = numbers.split_first().unwrap();
    is_possible_pt1(start * first, target, rest) || is_possible_pt1(start + first, target, rest)
}

fn concatenate(a: usize, b: usize) -> usize {
    a * 10_usize.pow(b.ilog10() + 1) + b
}

fn is_possible_pt2(start: usize, target: usize, numbers: &[usize]) -> bool {
    if numbers.is_empty() {
        return start == target;
    }

    if start > target {
        return false;
    }

    let (first, rest): (&usize, &[usize]) = numbers.split_first().unwrap();
    is_possible_pt2(start * first, target, rest)
        || is_possible_pt2(start + first, target, rest)
        || is_possible_pt2(concatenate(start, *first), target, rest)
}
