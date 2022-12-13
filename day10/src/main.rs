use std::fs::File;
use std::io;
use std::io::BufRead;
use crate::Instruction::{AddX, NoOp};

fn main() {
    let mut regx = 1;
    let file = File::open("./input.txt").unwrap();
    let instructions: Vec<_> = io::BufReader::new(file).lines().filter_map(|x| map_line(x.unwrap())).collect();

    let mut clock = 0;


    let mut instuction_next = None;
    let mut instructions = instructions.into_iter();
    loop {
        if clock % 40 == 0 {
            print!("\n");
        }
        if is_intersecting(regx as i64, clock % 40) {
            print!("#");
        }else {
            print!(".");
        }


        clock += 1;
        if let Some(AddX(i)) = &instuction_next {
            regx += i;
            instuction_next = None;
        }else {
            match instructions.next() {
                None => {
                    break;
                }
                Some(x) => {
                    instuction_next = Some(x);
                }
            }
        }



    }
}
fn is_intersecting(regx : i64, rowPos: i64) -> bool {
    (regx-1..=regx+1).contains(&rowPos)
}

fn map_line(i: String) -> Option<Instruction> {
    let mut parts = i.split_whitespace();

    let op = parts.next().unwrap();

    match op {
        "noop" => Some(NoOp),
        "addx" => Some(AddX(parts.next().unwrap().parse().unwrap())),
        _ => None
    }
}

enum Instruction {
    NoOp,
    AddX(i32),
}