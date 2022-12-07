use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Should have been able to read the file");
    let mut deers: Vec<i32> = Vec::new();
    let mut calories: i32 = 0;
    let mut deer_ranking: Vec<usize> = vec![0,0,0];
    for line in contents.lines() {
        if line == "" || line == "\n" {
            deers.push(calories);
//            println!("Deer #{}: {calories} cal",deers.len());
            let cur = deers.len() as usize - 1;
            if calories > deers[deer_ranking[2]] {
                if calories > deers[deer_ranking[0]] {
                    deer_ranking.insert(0, cur);
                    deer_ranking.pop();
                }
                else if calories > deers[deer_ranking[1]] {
                    deer_ranking.insert(1,cur);
                    deer_ranking.pop();
                }
                else {
                    deer_ranking[2] = cur;
                }
            }
            calories = 0;
        }
        else {
            calories += line.parse::<i32>().unwrap();
        }
    }
    println!("\nDeer ranking:");
    println!("#{}: {} cal", deer_ranking[0]+1, deers[deer_ranking[0]]);
    println!("#{}: {} cal", deer_ranking[1]+1, deers[deer_ranking[1]]);
    println!("#{}: {} cal", deer_ranking[2]+1, deers[deer_ranking[2]]);
    println!("Total: {} cal", deer_ranking.iter().map(|&x| deers[x]).sum::<i32>());
}