use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {

    let file = File::open("./input.txt").unwrap();

    let lines : Vec<_> = io::BufReader::new(file).lines().collect();

    let chars = lines.chunks(3)
        .map(|x| get_shared_char_pt2(&x[0].as_ref().unwrap(), &x[1].as_ref().unwrap(), &x[2].as_ref().unwrap()));

    let sum_of_points: i32 = chars.map(|x| get_points_for_char(x.unwrap())).sum();
    println!("{}", sum_of_points)
}
fn get_points_for_char(x : char) -> i32 {
    let scoring = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let array = scoring.chars().position(|c| c == x).unwrap() + 1;

    return array as i32
}
fn get_shared_char_pt2(a: &String, b: &String, c: &String) -> Option<char> {
    for char in a.chars() {
        if b.contains(char) && c.contains(char) {
            return Some(char)
        }
    }
    return None

}

fn get_shared_char(s: String) -> Option<char> {
    let split = s.split_at((s.len()/2) );

    for char in split.0.chars() {
        if split.1.contains(char) {
            return Some(char)
        }
    }
    return None

}
