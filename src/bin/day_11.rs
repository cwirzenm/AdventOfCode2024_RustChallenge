use cached::proc_macro::cached;

const INPUT: &str = include_str!("../../resources/day_11.txt");
const BLINKS_PT2: usize = 75;

fn main() {
    solve_pt1();
    solve_pt2();
}

fn parse_input() -> Vec<u64> {
    INPUT
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn solve_pt1() {
    let mut stones: Vec<u64> = parse_input();
    // println!("First state: {:?}", stones);

    for _ in 0..25 {
        let mut new_state = stones.clone();
        stones.iter().enumerate().rev().for_each(|(i, &elem)| {
            let n_digits: u64 = (elem.checked_ilog10().unwrap_or(0) + 1) as u64;
            if elem == 0 {
                new_state[i] = 1;
            } else if n_digits % 2 == 0 {
                let (left, right): (u64, u64) = split_digit(elem, n_digits);
                new_state[i] = right;
                new_state.insert(i, left);
            } else {
                new_state[i] = elem * 2024u64;
            }
        });
        // println!("New state: {:?}", new_state);
        stones = new_state.clone();
    }
    println!("Number of stones: {}", stones.len());
}

fn split_digit(n: u64, n_digits: u64) -> (u64, u64) {
    let half_digits: u32 = (n_digits / 2) as u32;
    let divisor: u64 = 10u64.pow(half_digits);
    (n / divisor, n % divisor)
}

fn solve_pt2() {
    let n_stones: usize = parse_input()
        .iter()
        .map(|stone| count_stones(*stone, 0))
        .sum();
    println!("Number of stones: {}", n_stones);
}

#[cached]
fn count_stones(stone: u64, current_blink: usize) -> usize {
    let mut count: usize = 1;
    let mut stone: u64 = stone;
    let mut current_blink: usize = current_blink;
    while current_blink < BLINKS_PT2 {
        current_blink += 1;
        let n_digits: u64 = (stone.checked_ilog10().unwrap_or(0) + 1) as u64;
        if stone == 0 {
            stone = 1;
        } else if n_digits % 2 != 0 {
            stone = stone * 2024;
        } else {
            let (left, right): (u64, u64) = split_digit(stone, n_digits);
            stone = left;
            count += count_stones(right, current_blink);
        }
    }
    count
}
