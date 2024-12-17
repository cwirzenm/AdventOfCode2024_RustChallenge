use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../resources/day_08.txt");

type MapOfAntennas = [Vec<Antenna>; u8::MAX as usize];

fn main() {
    let antennas: MapOfAntennas = parse_input();
    solve_pt1(&antennas);
    solve_pt2(&antennas);
}

fn solve_pt1(antennas: &MapOfAntennas) {
    let lines: isize = INPUT.lines().count() as isize;
    let mut count: usize = 0;
    HashSet::<Antinode>::from_iter(
        antennas
            .iter()
            .filter(|x: &&Vec<Antenna>| x.len() > 1)
            .flat_map(|a: &Vec<Antenna>| {
                a.iter()
                    .combinations(2)
                    .flat_map(|x: Vec<&Antenna>| {
                        calculate_antinodes(x)
                            .into_iter()
                            .filter(|node: &Antinode| within_bounds(&node, lines))
                    })
                    .collect::<Vec<Antinode>>()
            })
            .collect::<HashSet<Antinode>>(),
    )
    .iter()
    .for_each(|_| {
        count += 1;
    });
    println!("{}", count);
}

fn solve_pt2(antennas: &MapOfAntennas) {
    let lines: isize = INPUT.lines().count() as isize;
    let mut count: usize = 0;
    HashSet::<Antinode>::from_iter(
        antennas
            .iter()
            .filter(|x: &&Vec<Antenna>| x.len() > 1)
            .flat_map(|a: &Vec<Antenna>| {
                a.iter()
                    .combinations(2)
                    .flat_map(|x: Vec<&Antenna>| calculate_antinodes_pt2(x, lines))
                    .collect::<Vec<Antinode>>()
            })
            .collect::<HashSet<Antinode>>(),
    )
    .iter()
    .for_each(|_| {
        count += 1;
    });
    println!("{}", count);
}

fn parse_input() -> MapOfAntennas {
    let mut antennas: MapOfAntennas = [const { Vec::new() }; u8::MAX as usize];
    INPUT
        .split("\n")
        .into_iter()
        .enumerate()
        .for_each(|(i, line): (usize, &str)| {
            line.bytes()
                .enumerate()
                .filter(|(_, b)| b.is_ascii_alphanumeric())
                .for_each(|(j, b): (usize, u8)| {
                    antennas[b as usize].push(Antenna {
                        x: i as isize,
                        y: j as isize,
                    });
                })
        });
    antennas
}

fn calculate_antinodes(antenna_pair: Vec<&Antenna>) -> Vec<Antinode> {
    let dx: isize = antenna_pair[1].x - antenna_pair[0].x;
    let dy: isize = antenna_pair[1].y - antenna_pair[0].y;

    let first_antinode = Antinode {
        x: antenna_pair[0].x - dx,
        y: antenna_pair[0].y - dy,
    };
    let second_antinode = Antinode {
        x: antenna_pair[1].x + dx,
        y: antenna_pair[1].y + dy,
    };
    vec![first_antinode, second_antinode]
}

fn calculate_antinodes_pt2(antenna_pair: Vec<&Antenna>, len: isize) -> Vec<Antinode> {
    let dx: isize = antenna_pair[1].x - antenna_pair[0].x;
    let dy: isize = antenna_pair[1].y - antenna_pair[0].y;
    let mut n: isize = 0;
    let mut antinodes: Vec<Antinode> = Vec::new();
    loop {
        let node = Antinode {
            x: antenna_pair[0].x - (n * dx),
            y: antenna_pair[0].y - (n * dy),
        };
        if !within_bounds(&&node, len) {
            break;
        }
        antinodes.push(node);
        n += 1;
    }

    n = 0;
    loop {
        let node = Antinode {
            x: antenna_pair[1].x + (n * dx),
            y: antenna_pair[1].y + (n * dy),
        };
        if !within_bounds(&&node, len) {
            break;
        }
        antinodes.push(node);
        n += 1;
    }
    antinodes
}

fn within_bounds(pt: &&Antinode, len: isize) -> bool {
    let range = 0..len;
    range.contains(&pt.x) && range.contains(&pt.y)
}

#[derive(Debug)]
struct Antenna {
    x: isize,
    y: isize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Antinode {
    x: isize,
    y: isize,
}
