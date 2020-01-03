use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines() -> Vec<i32> {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut v = Vec::new();

    for l in reader.lines() {
        v = l.unwrap().split(",").map(|s| s.parse::<i32>().unwrap()).collect()
    }

    v
}

fn get_result(mut v: Vec<i32>, first: i32, second: i32) -> i32 {
    let mut curr_pos = 0;
    v[1] = first;
    v[2] = second;

    loop {
        if v[curr_pos as usize] == 99 {
            break;
        }

        let pos_1 = v[curr_pos as usize + 1];
        let pos_2 = v[curr_pos as usize + 2];

        if v[curr_pos as usize] == 1 {
            let sum_pos = v[curr_pos as usize + 3];
            v[sum_pos as usize] = v[pos_1 as usize] + v[pos_2 as usize];
        } else if v[curr_pos as usize] == 2 {
            let mul_pos = v[curr_pos as usize + 3];
            v[mul_pos as usize] = v[pos_1 as usize] * v[pos_2 as usize];
        }

        curr_pos += 4;
    }

    v[0]
}

fn main() {
    let v = read_lines();
    for first in (0..99) {
        for second in (0..99) {
            if get_result(v.clone(), first, second) == 19690720 {
                println!("first {}, second {}, result {}\n", first, second, 100 * first+second);
                return;
            }
        }
    }
}

