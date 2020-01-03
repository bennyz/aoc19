use std::fs::File;
use std::io::{BufRead, BufReader};

fn calc_fuel(mut f: i64) -> i64 {
    let mut sum = 0;

    loop {
        f = f / 3 - 2;
        if f <= 0 {
            break;
        }

        sum += f;
    }

    sum
}

fn read_lines() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let n = line.unwrap().parse::<i64>().unwrap();
        sum += calc_fuel(n);
    }

    println!("sum {}", sum);
}

fn main() {
    read_lines()
}

