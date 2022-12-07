use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");
    let mut final_score: u32 = 0;

    for line in contents.lines() {
        let first = &line[..line.len()/2];
        let second = &line[line.len()/2..];
        for a in first.chars() {
            if second.contains(a) {
                final_score += calculate_priority(a);
                break;
            }
        }
    }

    println!("Final score: {final_score}");
}

fn calculate_priority(ch:char) -> u32 {
    if ch.is_ascii_uppercase() {
        return ch as u32 - 65 + 27;
    }
    else if ch.is_ascii_lowercase() {
        return ch as u32 - 96;
    }
    else {
        return 0;
    }
}