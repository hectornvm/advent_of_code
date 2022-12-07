use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");
    let mut scores = HashMap::new();
    let mut final_score: u32 = 0;

    // Rock: 1, Paper: 2, Scissors: 3
    // Win: 6, Tie: 3, Loss: 0
    // X: Loss, Y: Tie, : Z: Win
    scores.insert("A X", 3); // Rock vs Rock
    scores.insert("A Y", 4); // Rock vs Paper
    scores.insert("A Z", 8); // Rock vs Scissors
    scores.insert("B X", 1); // Paper vs Rock
    scores.insert("B Y", 5); // Paper vs Paper
    scores.insert("B Z", 9); // Paper vs Scissors
    scores.insert("C X", 2); // Scissors vs Rock
    scores.insert("C Y", 6); // Scissors vs Paper
    scores.insert("C Z", 7); // Scissors vs Scissors

    for line in contents.lines() {
        final_score += scores[line];
    }

    println!("Final score: {final_score}");
}