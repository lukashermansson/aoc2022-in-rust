extern crate core;


use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use crate::End::{Draw, Loose, Win};
use crate::Move::{Paper, Rock, Scissors};

fn main() {
    let path = Path::new("./input.txt");
    let display = path.display();
    println!("{}", display);

    let file = File::open(&path).unwrap();

    let lines = io::BufReader::new(file).lines();
    let mut playes = vec![];

    for line in lines {
        let line = line.unwrap();
        let mut split = line.split(" ");

        let opponent = split.next().unwrap();
        let mine = split.next().unwrap();

        let opponent_move = match opponent {
            "A" => Some(Rock),
            "B" => Some(Paper),
            "C" => Some(Scissors),
            _ => None
        }.unwrap();

        let end = match mine {
            "X" => Some(Loose),
            "Y" => Some(Draw),
            "Z" => Some(Win),
            _ => None
        }.unwrap();
        playes.push((opponent_move, end))
    }
    println!("{:?}", playes);
    let result: i32 = playes.iter().map(|x| get_points(x)).sum();
    println!("{}", result)
}
fn get_points(input: &(Move, End)) -> i32 {
    let mut round_total = 0;

    if input.1 == Draw {
        round_total += 3;

        match input.0 {
            Rock => round_total += 1,
            Scissors => round_total += 3,
            Paper => round_total += 2,
        };
    }
    if input.1 == Loose {
        round_total += 0;

        match input.0 {
            Rock => round_total += 3,
            Scissors => round_total += 2,
            Paper => round_total += 1,
        };
    }
    if input.1 == Win {
        round_total += 6;

        match input.0 {
            Rock => round_total += 2,
            Scissors => round_total += 1,
            Paper => round_total += 3,
        };
    }

    println!("{}", round_total);

    return round_total;
}

#[derive(PartialEq, Debug)]
enum Move {
    Rock,
    Scissors,
    Paper
}
#[derive(PartialEq, Debug)]
enum End {
    Win,
    Draw,
    Loose
}