use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

enum Direction {
    R,
    L,
    U,
    D,
}

struct Path {
    dir: Direction,
    count: i32,
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
    steps: i32,
}

// Implement custom hash, since equal positions will have different
// hashes because the steps do not match
impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

fn process_lines() {
    let mut wire1: Vec<Position> = Vec::new();
    let mut wire2: Vec<Position> = Vec::new();

    let content = fs::read_to_string("input").unwrap();
    let content: Vec<&str> = content.split("\n").collect();

    place_wire(&mut wire1, content[0].split(",").collect());
    place_wire(&mut wire2, content[1].split(",").collect());

    let s_1: HashSet<Position> = HashSet::from_iter(wire1.iter().cloned());
    let s_2: HashSet<Position> = HashSet::from_iter(wire2.iter().cloned());

    let part1 = s_1
        .intersection(&s_2)
        .map(|pos| (pos.x - 0).abs() + (pos.y - 0).abs())
        .min()
        .unwrap();
    println!("{}", part1);

    let part2 = s_1
        .intersection(&s_2)
        .map(|pos| s_1.get(pos).unwrap().steps + s_2.get(pos).unwrap().steps)
        .min()
        .unwrap();
    println!("{}", part2);
}

fn place_wire(wire: &mut Vec<Position>, instructions: Vec<&str>) {
    let mut curr_pos = Position {
        x: 0,
        y: 0,
        steps: 0,
    };
    for s in instructions {
        let path = get_path(s);

        for _ in 0..path.count {
            curr_pos = match path.dir {
                Direction::R => Position {
                    x: curr_pos.x + 1,
                    y: curr_pos.y + 0,
                    steps: curr_pos.steps,
                },
                Direction::L => Position {
                    x: curr_pos.x - 1,
                    y: curr_pos.y + 0,
                    steps: curr_pos.steps,
                },
                Direction::U => Position {
                    x: curr_pos.x + 0,
                    y: curr_pos.y + 1,
                    steps: curr_pos.steps,
                },
                Direction::D => Position {
                    x: curr_pos.x + 0,
                    y: curr_pos.y - 1,
                    steps: curr_pos.steps,
                },
                // Shouldn't happen
                _ => Position {
                    x: curr_pos.x + 0,
                    y: curr_pos.y + 0,
                    steps: curr_pos.steps,
                },
            };

            curr_pos.steps += 1;
            wire.push(curr_pos);
        }
    }
}

fn get_path(command: &str) -> Path {
    let op = command.chars().nth(0).unwrap();
    let val = command[1..].parse::<i32>().unwrap();

    let op = match op {
        'R' => Direction::R,
        'L' => Direction::L,
        'U' => Direction::U,
        'D' => Direction::D,
        _ => panic!("poo"),
    };

    Path {
        dir: op,
        count: val,
    }
}

fn main() {
    process_lines();
}
