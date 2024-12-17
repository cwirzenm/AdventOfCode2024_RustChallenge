static TARGET: &str = "XMAS";
static TARGET_REV: &str = "SAMX";

const INPUT: &str = include_str!("../../resources/day_04.txt");

fn main() {
    solve_pt1(INPUT);
    solve_pt2(INPUT);
}

fn horizontal(lines: &Vec<&str>) -> usize {
    let mut count: usize = 0;

    for line in lines {
        for i in 0..(line.len() - 3) {
            if &line[i..(i + 4)] == TARGET || &line[i..(i + 4)] == TARGET_REV {
                count += 1;
            }
        }
    }
    count
}

fn vertical(lines: &Vec<&str>) -> usize {
    let height: usize = lines.len();
    let width: usize = lines.first().unwrap().len();
    let mut count: usize = 0;

    for row in 0..(height - 3) {
        for col in 0..width {
            let mut vertical_str = String::new();

            for i in 0..4 {
                let char: char = lines[row + i].chars().nth(col).unwrap();
                vertical_str.push(char);
            }

            if vertical_str == TARGET || vertical_str == TARGET_REV {
                count += 1;
            }
        }
    }
    count
}

fn diagonal(lines: &Vec<&str>) -> usize {
    let height: usize = lines.len();
    let width: usize = lines.first().unwrap().len();
    let mut count: usize = 0;

    for row in 0..(height - 3) {
        for col in 0..(width - 3) {
            let mut vertical_str: String = String::new();

            for i in 0..4 {
                let char = lines[row + i].chars().nth(col + i).unwrap();
                vertical_str.push(char);
            }

            if vertical_str == TARGET || vertical_str == TARGET_REV {
                count += 1;
            }
        }
        for col in 3..width {
            let mut vertical_str: String = String::new();

            for i in 0..4 {
                let char = lines[row + i].chars().nth(col - i).unwrap();
                vertical_str.push(char);
            }

            if vertical_str == TARGET || vertical_str == TARGET_REV {
                count += 1;
            }
        }
    }
    count
}

fn solve_pt1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut count: usize = 0;

    count += horizontal(&lines);
    count += vertical(&lines);
    count += diagonal(&lines);
    println!("{:?}", count);

    count
}

fn solve_pt2(input: &str) -> usize {
    let target: &str = "MAS";
    let target_rev: &str = "SAM";

    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines.first().unwrap().len();
    let mut count: usize = 0;

    for row in 0..(height - 2) {
        for col in 0..(width - 2) {
            let mut diag1 = String::new();
            let mut diag2 = String::new();

            for i in 0..3 {
                let char: char = lines[row + i].chars().nth(col + i).unwrap();
                diag1.push(char);

                let char: char = lines[row + i].chars().nth(col + 2 - i).unwrap();
                diag2.push(char);
            }

            if (diag1 == target || diag1 == target_rev) && (diag2 == target || diag2 == target_rev) {
                count += 1;
            }
        }
    }

    println!("{:?}", count);
    count
}
