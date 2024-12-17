use std::collections::HashMap;

const INPUT: &str = include_str!("../../resources/day_01.txt");

fn main() {
    let split: Vec<&str> = INPUT.split("\n").collect::<Vec<&str>>();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for pair in split {
        let line: Vec<&str> = pair.split("   ").collect::<Vec<&str>>();
        left.push(to_int(line[0]));
        right.push(to_int(line[1]));
    }
    left.sort();
    right.sort();

    let mut difference: i32 = 0;
    let it = left.iter().zip(right.iter());
    for (_i, (x, y)) in it.enumerate() {
        difference += (x - y).abs()
    }
    println!("difference: {:?}", difference);

    let mut similarity: i32 = 0;
    let mut lookup: HashMap<i32, i32> = HashMap::new();
    for l in left {
        if lookup.contains_key(&l) {
            similarity += lookup.get(&l).unwrap();
        } else {
            let count: i32 = right
                .iter()
                .filter(|&n| *n == l)
                .count() as i32;
            similarity += l * count;
            lookup.insert(l, l * count);
        }
    }
    println!("similarity: {:?}", similarity);
}

fn to_int(str: &str) -> i32 {
    str.parse::<i32>().unwrap()
}
