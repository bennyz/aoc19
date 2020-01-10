use std::convert::TryFrom;
use std::iter::FromIterator;

fn adjacent_pair(n: u32) -> bool {
    let v = Vec::from_iter(n.to_string().chars());
    let mut occ: [u32; 10] = [0; 10];

    for i in v {
        // There has to be a better way...
        let i = i.to_digit(10).unwrap();
        occ[usize::try_from(i).unwrap()] += 1;
    }

    for i in &occ {
        if *i == 2 {
            return true;
        }
    }

    false
}

fn only_increasing(n: u32) -> bool {
    let v = Vec::from_iter(n.to_string().chars());

    for i in 0..v.len() - 1 {
        if v[i] as u32 > v[i + 1] as u32 {
            return false;
        }
    }

    true
}

fn check_number(n: u32) -> bool {
    if n < 100000 || n > 999999 {
        return false;
    }

    if !only_increasing(n) {
        return false;
    }

    if !adjacent_pair(n) {
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
