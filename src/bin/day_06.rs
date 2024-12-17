use rayon::prelude::*;
use std::{collections::HashSet, iter::once};

const INPUT: &str = include_str!("../../resources/day_06.txt");

fn main() {
    solve_pt1(INPUT);
    solve_pt2(INPUT);
}

fn solve_pt1(input: &str) {
    let grid: GuardPatrolMap = GuardPatrolMap::new(input);
    let visited: HashSet<Position> = HashSet::<Position>::from_iter(
        once(grid.guard.pos)
            .chain(grid.map(|g| g.pos))
            .collect::<HashSet<Position>>(),
    );
    println!("Tiles visited by the guard: {}", visited.len());
}

fn solve_pt2(input: &str) {
    let grid: GuardPatrolMap = GuardPatrolMap::new(input);
    let starting_pos: Position = grid.guard.pos;
    let mut visited: HashSet<Position> =
        HashSet::<Position>::from_iter(grid.clone().map(|g| g.pos).collect::<HashSet<Position>>());
    visited.remove(&starting_pos);

    let count: usize = visited
        .into_iter()
        .par_bridge()
        .map(|v: Position| -> usize {
            if grid.add_new_obstacle(v).guard_in_loop() {
                1
            } else {
                0
            }
        })
        .sum();
    println!("There are {} combinations where guard is in infinite loop", count);
}

type Position = (usize, usize);

#[derive(Clone, PartialEq, Hash, Eq, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    dir: Direction,
    pos: Position,
}

impl Guard {
    fn init(pos: Position) -> Self {
        Self {
            dir: Direction::Up,
            pos,
        }
    }

    fn new_state(&mut self, obstacles: &Vec<Position>) -> Option<Self> {
        let (x, y) = self.pos;
        let new_pos: Position = match self.dir {
            Direction::Up => (x.checked_sub(1)?, y),
            Direction::Down => (x + 1, y),
            Direction::Left => (x, y.checked_sub(1)?),
            Direction::Right => (x, y + 1),
        };

        if obstacles.contains(&new_pos) {
            self.dir.turn_right();
            return Some(*self);
        }
        self.pos = new_pos;
        Some(*self)
    }
}

#[derive(Clone, Debug)]
struct GuardPatrolMap {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
    guard: Guard,
    obstacles: Vec<Position>,
    collisions: HashSet<(Position, Direction)>,
}

impl GuardPatrolMap {
    fn new(input: &str) -> Self {
        let mut tiles: Vec<Vec<char>> = input
            .lines()
            .map(|line: &str| line.chars().collect())
            .collect();
        let width: usize = tiles[0].len();
        let height: usize = tiles.len();
        let mut guard_pos: Option<Position> = None;
        let mut obstacles: Vec<Position> = Vec::new();
        let collisions: HashSet<(Position, Direction)> = HashSet::new();

        for i in 0..width {
            for j in 0..height {
                if tiles[i][j] == '^' {
                    if let Some(_val) = guard_pos {
                        panic!("Second guard found")
                    }
                    guard_pos = Option::from((i, j));
                    tiles[i][j] = 'X'
                }
                if tiles[i][j] == '#' {
                    obstacles.push((i, j));
                }
            }
        }

        if let None = guard_pos {
            panic!("Guard not found")
        }

        Self {
            tiles,
            width,
            height,
            guard: Guard::init(guard_pos.unwrap()),
            obstacles,
            collisions,
        }
    }

    fn next_index(&mut self) -> Option<Guard> {
        let i: Guard = self.guard.new_state(&self.obstacles)?;
        ((0..self.width).contains(&i.pos.0) && (0..self.height).contains(&i.pos.1)).then_some(i)
    }

    fn mark_position(&mut self, pos: Position) {
        self.tiles[pos.0][pos.1] = 'X';
        // for i in 0..self.width {
        //     println!("{:?}", self.tiles[i]);
        // }
        // println!("\n")
    }

    fn add_new_obstacle(&self, pos: Position) -> GuardPatrolMap {
        let mut new: GuardPatrolMap = self.clone();
        new.obstacles.push(pos);
        new
    }

    fn guard_in_loop(&mut self) -> bool {
        if let Some(_final_state) = self.last() {
            if let Some(_next_state) = self.next_index() {
                // if the next state is possible then we're not out of bounds which means we're in a loop
                return true;
            }
        }
        false
    }
}

impl Iterator for GuardPatrolMap {
    type Item = Guard;
    fn next(&mut self) -> Option<Self::Item> {
        let previous_pos: Position = self.guard.pos.clone();
        let new_state: Guard = self.next_index()?;
        let new_pos: Position = new_state.pos;
        if previous_pos == new_pos {
            if !self.collisions.insert((new_state.pos, new_state.dir)) {
                // if a second collision happens in the same state then the guard is in an infinite loop
                return None;
            };
        } else {
            self.mark_position(new_pos)
        };
        Some(self.guard)
    }
}
