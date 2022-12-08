use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");
    let mut redundant_pairs: u32 = 0;

    for line in contents.lines() {
        let pair: Vec<u8> = line.split(&[',', '-'][..]).map(|x| x.parse::<u8>().unwrap()).collect();
        if (pair[0] <= pair[2] && pair[1] >= pair[3]) ||
           (pair[0] >= pair[2] && pair[1] <= pair[3]) {
            redundant_pairs += 1;
        }
    }

    println!("Redundant pairs: {redundant_pairs}");
}
