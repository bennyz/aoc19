use std::iter::FromIterator;

fn only_increasing_and_adjacent(n: u32) -> bool {
    let v = Vec::from_iter(n.to_string().chars());
    let mut adjacent = false;

    for i in 0..v.len() - 1 {
        if v[i] > v[i + 1] {
            return false;
        }

        if v[i] == v[i + 1] {
            adjacent = true;
        }
    }

    adjacent
}

fn check_number(n: u32) -> bool {
    if n < 100000 || n > 999999 {
        return false;
    }

    if !only_increasing_and_adjacent(n) {
        return false;
    }

    true
}

fn main() {
    let input = "356261-846303";
    let range: Vec<&str> = input.split("-").collect();

    let lower = range[0].parse::<u32>().unwrap();
    let upper = range[1].parse::<u32>().unwrap();

    let mut count = 0;
    for n in lower..upper {
        if check_number(n) {
            count += 1;
        }
    }

    println!("{}", count);
}
