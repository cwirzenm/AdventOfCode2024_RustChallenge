use std::{cmp::Ordering, collections::HashSet};

const INPUT: &str = include_str!("../../resources/day_05.txt");

fn main() {
    let (orderings, pages): (&str, &str) = INPUT.split_once("\n\n").unwrap();

    let orderings: HashSet<(usize, usize)> = orderings
        .lines()
        .map(|line: &str| {
            let split: (&str, &str) = line.split_once("|").unwrap();
            (
                split.0.parse::<usize>().unwrap(),
                split.1.parse::<usize>().unwrap(),
            )
        })
        .collect::<HashSet<(usize, usize)>>();

    solve_pt1(&orderings, pages);
    solve_pt2(&orderings, pages);
}

fn solve_pt1(orderings: &HashSet<(usize, usize)>, pages: &str) {
    let compare = |x: &usize, y: &usize| !orderings.contains(&(*y, *x));

    let mut count = 0;
    for update in pages.lines() {
        let update: Vec<usize> = update
            .split(',')
            .map(|x: &str| x.parse().unwrap())
            .collect();

        if update.is_sorted_by(compare) {
            count += update[update.len() / 2];
        }
    }

    println!("{}", count);
}

fn solve_pt2(orderings: &HashSet<(usize, usize)>, pages: &str) {
    let compare = |x: &usize, y: &usize| {
        if orderings.contains(&(*y, *x)) {
            Ordering::Greater
        } else if orderings.contains(&(*x, *y)) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    };

    let mut count = 0;
    for update in pages.lines() {
        let mut update: Vec<usize> = update
            .split(',')
            .map(|x: &str| x.parse().unwrap())
            .collect();

        if !update.is_sorted_by(|a: &usize, b: &usize| compare(a, b) != Ordering::Greater) {
            update.sort_by(compare);
            count += update[update.len() / 2];
        }
    }

    println!("{}", count);
}
