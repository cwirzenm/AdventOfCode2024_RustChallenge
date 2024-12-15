use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("../../resources/day_03.txt");
    let re: Regex = Regex::new(r"mul\((?<num1>\d+),(?<num2>\d+)\)").unwrap();

    println!("Part 1: {}", solve_pt1(INPUT, &re));
    println!("Part 2: {}", solve_pt2(INPUT));
}

fn solve_pt1(input: &str, re: &Regex) -> isize {
    re.captures_iter(input)
        .map(|c| {
            let num1 = c.name("num1").unwrap().as_str().parse().unwrap();
            let num2 = c.name("num2").unwrap().as_str().parse().unwrap();
            (num1, num2)
        })
        .collect::<Vec<(isize, isize)>>()
        .iter()
        .map(|(num1, num2)| num1 * num2)
        .sum()
}

enum Toggle {
    On,
    Off,
}

impl From<&str> for Toggle {
    fn from(value: &str) -> Self {
        match value {
            "do" => Toggle::On,
            "don't" => Toggle::Off,
            _ => panic!(),
        }
    }
}

fn solve_pt2(input: &str) -> isize {
    let toggle_re: Regex =
        Regex::new(r"(?<toggle>don't|do)|mul\((?<num1>\d+),(?<num2>\d+)\)").unwrap();

    let mut toggle = Toggle::On;
    let mut total = 0;

    toggle_re
        .captures_iter(input)
        .for_each(|c| {
            let t = c.name("toggle");
            match t {
                None => {
                    if let Toggle::On = toggle {
                        let num1: isize = c.name("num1").unwrap().as_str().parse().unwrap();
                        let num2: isize = c.name("num2").unwrap().as_str().parse().unwrap();
                        total += num1 * num2;
                    }
                }
                Some(_) => {
                    toggle = Toggle::from(t.unwrap().as_str());
                }
            }
        });
    total
}
