use std::fs;

fn main() {
    println!(
        "{}",
        fs::read_to_string("./input.txt")
            .expect("Failed to get input!")
            .split("\n\n")
            .map(|g| g.lines().map(|l| l.parse::<i32>().unwrap()).sum::<i32>())
            .max()
            .unwrap()
    );
}
