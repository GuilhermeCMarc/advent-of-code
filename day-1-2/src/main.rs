use std::fs;

fn main() {
    let mut max_vec = fs::read_to_string("./input.txt")
        .expect("Failed to get input!")
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.parse::<i32>().unwrap()).sum::<i32>())
        .collect::<Vec<i32>>();

    max_vec.sort();

    let mut sum = 0;
    for x in (max_vec.len() - 3)..max_vec.len() {
        sum += max_vec[x];
    }

    println!("{sum}");
}
