use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");
    let mut deers = Vec::new();
    let mut calories: i32 = 0;
    let mut max_calories_deer = 0;
    for line in contents.lines() {
        if line == "" || line == "\n" {
            deers.push(calories);
            println!("Deer #{}: {calories} cal",deers.len());
            if calories > deers[max_calories_deer] {
                max_calories_deer = deers.len()-1
            }
            calories = 0;
        }
        else {
            calories += line.parse::<i32>().unwrap();
        }
    }
    println!("Deer with the most calories is #{} with {} cal",max_calories_deer+1, deers[max_calories_deer])
}