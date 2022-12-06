use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("./input.txt");
    let display = path.display();
    println!("{}", display);

    let file = File::open(&path).unwrap();

    let lines = io::BufReader::new(file).lines();

    let mut calories = Vec::new();

    let mut current_calories = 0;
    for line in lines {
        match line {
            Ok(x) => {
                if x.is_empty() {
                    calories.push(current_calories);
                    current_calories = 0
                }else {
                    let as_int: i32 = x.parse().unwrap();
                    current_calories += as_int;
                }
            },
            Err(_) => {

            }
        }
    }
    calories.sort_by(|a, b| b.cmp(a));

    calories.truncate(3);
    let top3value: i32 = calories.iter().sum();

    println!("{:?}", top3value)
}
