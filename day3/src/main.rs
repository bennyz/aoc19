use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs;

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

#[derive(Debug, Copy, Clone, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}


fn process_lines() -> i32 {
    let mut wire1: Vec<Position> = Vec::new();
    let mut wire2: Vec<Position> = Vec::new();

    let content = fs::read_to_string("input").unwrap();
    let content: Vec<&str> = content.split("\n").collect();

    place_wire(&mut wire1, content[0].split(",").collect());
    place_wire(&mut wire2, content[1].split(",").collect());

    let s_1: HashSet<Position> = HashSet::from_iter(wire1.iter().cloned());
    let s_2: HashSet<Position> = HashSet::from_iter(wire2.iter().cloned());

    s_1.intersection(&s_2)
        .map(|pos| (pos.x - 0).abs() + (pos.y - 0).abs())
        .min()
        .unwrap()
}

fn place_wire(wire: &mut Vec<Position>, instructions: Vec<&str>) {
    let mut curr_pos = Position { x: 0, y: 0 };
    for s in instructions {
        let path = get_path(s);
        for _ in 0..path.count {
            curr_pos = match path.dir {
                Direction::R => Position {
                    x: curr_pos.x + 1,
                    y: curr_pos.y + 0,
                },
                Direction::L => Position {
                    x: curr_pos.x - 1,
                    y: curr_pos.y + 0,
                },
                Direction::U => Position {
                    x: curr_pos.x + 0,
                    y: curr_pos.y + 1,
                },
                Direction::D => Position {
                    x: curr_pos.x + 0,
                    y: curr_pos.y - 1,
                },
                // Shouldn't happen
                _ => Position {
                    x: curr_pos.x + 0,
                    y: curr_pos.y + 0,
                },
            };

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
    println!("{}", process_lines());
}
