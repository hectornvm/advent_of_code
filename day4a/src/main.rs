use std::fs;
use std::cmp;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");
    let mut overlapping_pairs: u32 = 0;

    for line in contents.lines() {
        let pair: Vec<u8> = line.split(&[',', '-'][..]).map(|x| x.parse::<u8>().unwrap()).collect();
        if (cmp::max(pair[1], pair[3]) - cmp::min(pair[0],pair[2])) <= (pair[1]-pair[0]+pair[3]-pair[2]) {
            overlapping_pairs += 1;
//            println!("Overlapping pair #{overlapping_pairs}: {line}");
        }
    }

    println!("Overlapping pairs: {overlapping_pairs}");
}
