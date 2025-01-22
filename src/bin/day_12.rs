use rustc_hash::FxHashSet;
use std::collections::VecDeque;

type Grid<T> = Vec<Vec<T>>;

const INPUT: &str = include_str!("../../resources/day_12.txt");
const DIR: [(i32, i32); 4] = [
    (1, 0),
    (0, 1),
    (0, -1),
    (-1, 0),
];

fn main() {
    solve_pt1();
    solve_pt2();
}

fn solve_pt1() {
    let input: Grid<char> = parse_map();
    let mut visited = FxHashSet::default();
    let mut price: usize = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if !visited.contains(&(i as i32, j as i32)) {
                let (region_area, region_perim) = bfs((i as i32, j as i32), &mut visited, &input);
                price += region_area.len() * region_perim;
            }
        }
    }
    println!("Price: {}", price);
}

fn solve_pt2() {
    let input = parse_map();
    let mut visited = FxHashSet::default();
    let mut price = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if !visited.contains(&(i as i32, j as i32)) {
                let (region_area, _) = bfs((i as i32, j as i32), &mut visited, &input);
                let sides = count_region_sides(&region_area);
                price += region_area.len() * sides;
            }
        }
    }
    println!("Price: {}", price);
}

fn parse_map() -> Grid<char> {
    let mut map: Grid<char> = Vec::new();
    INPUT.lines().for_each(|line: &str| {
        map.push(line.chars().collect());
    });
    map
}

fn get_neighbors(pos: (i32, i32), map: &Vec<Vec<char>>, plant: char) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::new();

    for dir in DIR {
        let neighbor_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if (neighbor_pos.0 as usize) < map.len() && (neighbor_pos.1 as usize) < map[0].len() && map[neighbor_pos.0 as usize][neighbor_pos.1 as usize] == plant {
            neighbors.push(neighbor_pos);
        }
    }

    neighbors
}

fn bfs(pos: (i32, i32), visited: &mut FxHashSet<(i32, i32)>, map: &Vec<Vec<char>>) -> (FxHashSet<(i32, i32)>, usize) {
    let mut queue = VecDeque::new();
    let plant: char = map[pos.0 as usize][pos.1 as usize];
    let mut area = FxHashSet::default();
    let mut perimiter: usize = 0;
    visited.insert(pos);
    queue.push_back(pos);
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        area.insert(curr);
        let neighbors = get_neighbors(curr, map, plant);
        perimiter += 4 - neighbors.len();
        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    (area, perimiter)
}

fn count_region_sides(region: &FxHashSet<(i32, i32)>) -> usize {
    let mut side_count: usize = 0;
    for dir in DIR {
        let mut sides = FxHashSet::default();
        for pos in region {
            let tmp = (pos.0 + dir.0, pos.1 + dir.1);
            if !region.contains(&tmp) {
                sides.insert(tmp);
            }
        }
        let mut remove = FxHashSet::default();
        for side in &sides {
            let mut tmp = (side.0 + dir.1, side.1 + dir.0);
            while sides.contains(&tmp) {
                remove.insert(tmp);
                tmp = (tmp.0 + dir.1, tmp.1 + dir.0);
            }
        }
        side_count += sides.len() - remove.len();
    }

    side_count
}
