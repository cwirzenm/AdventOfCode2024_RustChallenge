use std::collections::HashSet;

type Map = Vec<Vec<char>>;

const INPUT: &str = include_str!("../../resources/day_10.txt");

fn main() {
    let map: Map = parse_map();
    solve_pt1(&map);
    solve_pt2(&map);
}

fn solve_pt1(map: &Map) {
    let trail_heads: Vec<Coords> = find_trail_heads(map);

    let mut total_score: usize = 0;
    for trail_head in trail_heads {
        let mut trail_score: usize = 0;
        let mut visited: HashSet<Coords> = HashSet::new();
        walk_trail(map, &trail_head, &mut trail_score, &mut visited);
        total_score += trail_score;
    }
    println!("Total score: {}", total_score);
}

fn solve_pt2(map: &Map) {
    let trail_heads: Vec<Coords> = find_trail_heads(map);

    let mut total_score: usize = 0;
    for trail_head in trail_heads {
        let mut trail_score: usize = 0;
        walk_trail_pt2(map, &trail_head, &mut trail_score);
        total_score += trail_score;
    }
    println!("Total score: {}", total_score);
}

fn parse_map() -> Map {
    let mut map: Map = Vec::new();
    for line in INPUT.lines() {
        map.push(line.chars().collect());
    }
    map
}

fn find_trail_heads(map: &Map) -> Vec<Coords> {
    let mut list_of_coords: Vec<Coords> = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col.eq(&'0') {
                list_of_coords.push(Coords {
                    x: x as isize,
                    y: y as isize,
                    z: 0,
                });
            }
        }
    }
    list_of_coords
}

fn walk_trail(
    map: &Map,
    curr_pos: &Coords,
    trail_score: &mut usize,
    visited: &mut HashSet<Coords>,
) {
    if !visited.insert(curr_pos.clone()) {
        return;
    }
    if curr_pos.z == 9 {
        *trail_score += 1;
        return;
    }
    let next: Vec<Coords> = curr_pos.find_next(map);
    for path in next {
        walk_trail(map, &path, trail_score, visited);
    }
}

fn walk_trail_pt2(map: &Map, curr_pos: &Coords, trail_score: &mut usize) {
    if curr_pos.z == 9 {
        *trail_score += 1;
        return;
    }
    let next: Vec<Coords> = curr_pos.find_next(map);
    for path in next {
        walk_trail_pt2(map, &path, trail_score);
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Coords {
    x: isize,
    y: isize,
    z: isize,
}

impl Coords {
    fn new(map: &Map, x: isize, y: isize) -> Option<Self> {
        if x < 0 || y < 0 || x >= map.len() as isize || y >= map[0].len() as isize {
            None
        } else {
            Some(Self {
                x,
                y,
                z: Coords::get_height(map, x, y),
            })
        }
    }

    fn get_height(map: &Map, x: isize, y: isize) -> isize {
        map[y as usize][x as usize].to_digit(10).unwrap() as isize
    }

    fn push_if_some(list: &mut Vec<Coords>, coords: Option<Coords>) {
        if let Some(c) = coords {
            list.push(c);
        }
    }

    fn get_neighbors(&self, map: &Map) -> Vec<Coords> {
        let mut list_of_coords: Vec<Coords> = Vec::new();
        Coords::push_if_some(&mut list_of_coords, Coords::new(map, self.x + 1, self.y));
        Coords::push_if_some(&mut list_of_coords, Coords::new(map, self.x - 1, self.y));
        Coords::push_if_some(&mut list_of_coords, Coords::new(map, self.x, self.y + 1));
        Coords::push_if_some(&mut list_of_coords, Coords::new(map, self.x, self.y - 1));
        list_of_coords
    }

    fn is_valid(&self, path: &Coords) -> bool {
        path.z - self.z == 1
    }

    fn find_next(&self, map: &Map) -> Vec<Coords> {
        let mut list_of_paths: Vec<Self> = Vec::new();
        for path in self.get_neighbors(map) {
            if self.is_valid(&path) {
                list_of_paths.push(path);
            }
        }
        list_of_paths
    }
}
