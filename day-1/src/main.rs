use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to get input!");

    let mut max_vec: Vec<i32> = input
        .split("\n\n")
        .map(|group| {
            let sum = group
                .split("\n")
                .map(|row| row.parse::<i32>().unwrap())
                .sum();
            return sum;
        })
        .collect();

    max_vec.sort();

    let max = max_vec.last().unwrap();

    println!("{max}");
}
