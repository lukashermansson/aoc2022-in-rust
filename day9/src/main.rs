use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::SplitWhitespace;
use crate::Direction::{Down, Left, Right, Up};

fn main() {

    let file = File::open("./input.txt").unwrap();

    let moves : Vec<_> = io::BufReader::new(file).lines().filter_map(|x| map_line(x.unwrap())).collect();

    let mut visited_tiles : HashSet<Coord> = HashSet::new();


    let mut rope = [Coord {
        x: 100,
        y: 100
    }; 10];


    for m in moves {
        for n in 0..m.amount {
            for i in 0..rope.len() {
                let mut c = rope[i];
                if i == 0 {

                    match m.dir {
                        Up => c.y = c.y - 1,
                        Right => c.x = c.x + 1,
                        Left => c.x = c.x - 1,
                        Down => c.y = c.y + 1
                    }
                    rope[i] = c;
                }else {
                    rope[i] = get_new_pos_of_segment(&rope[i - 1], &c);
                }
            }
            let posOfLast = rope.last().unwrap().clone();
            visited_tiles.insert(posOfLast);
        }

    }

    println!("{}", visited_tiles.len())


}
fn dbgprint(head: &Coord, tail: &Coord, m: &Move) {
    println!("{:?}", m);

    println!("{:?}", head);
    println!("{:?}", tail);
}

fn get_new_pos_of_segment(prev_segment: &Coord, current_seg: &Coord) -> Coord {
    //Touching
    if (current_seg.x >= prev_segment.x-1 && current_seg.x <= prev_segment.x+1) &&
        (current_seg.y >= prev_segment.y-1 && current_seg.y <= prev_segment.y+1) {
        return Coord {
            x: current_seg.x,
            y: current_seg.y
        }
    }
    let x_diff = current_seg.x - prev_segment.x;
    let y_diff = current_seg.y - prev_segment.y;
    return Coord {
        x: if x_diff == 0 { current_seg.x } else { if x_diff < 0 { current_seg.x + 1} else { current_seg.x - 1} },
        y: if y_diff == 0 { current_seg.y } else { if y_diff < 0 { current_seg.y + 1} else { current_seg.y - 1} },
    }
}

fn map_line(input: String) -> Option<Move> {
    let mut parts = input.split_whitespace();
    match (parts.next(), parts.next()) {
        (Some(dir), Some(amt)) => {
            let dir = match dir {
                "U" => Some(Up),
                "D" => Some(Down),
                "R" => Some(Right),
                "L" => Some(Left),
                _ => None
             }?;

            Some(Move {
                dir,
                amount: amt.parse().ok()?,
            })
        }
        _ => None
    }
}
#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Coord {
    x: i128,
    y: i128
}
#[derive(Debug)]
struct Move {
    dir: Direction,
    amount: i128
}
#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Left,
    Down
}