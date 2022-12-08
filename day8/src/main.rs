extern crate core;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::slice::SliceIndex;

fn main() {

    let file = File::open("./input.txt").unwrap();

    let lines : Vec<_> = io::BufReader::new(file).lines().collect();

    let mut yvec = vec![];
    for y in lines {
        let mut xvec = vec![];
        for x in y.unwrap().chars() {
            let x = x.to_digit(10).unwrap();
            xvec.push(x);
        }
        yvec.push(xvec)
    }

    let forest = Forest {
        trees: yvec,
    };

    let mut total_trees = 0;
    for y in forest.trees.iter().enumerate() {
        for x in y.1.iter().enumerate() {
            if forest.is_visible_from_outside((x.0, y.0)) {
                total_trees += 1;
            }
        }
    }
    println!("{}", total_trees);

    let mut higest = 0;
    for y in forest.trees.iter().enumerate() {
        for x in y.1.iter().enumerate() {
            let score = forest.get_scenic_score((x.0, y.0));

            if(score > higest) {
                higest = score
            }
        }
    }
    println!("{}", higest);
}
struct Forest {
    trees: Vec<Vec<u32>>
}

impl Forest {
    fn is_visible_from_outside(&self, (x, y): (usize, usize)) -> bool {
        let tree : &u32 = self.trees.get(y).unwrap().get(x).unwrap();


        //check horizontal
        for lijne in self.trees.get(y) {
            if lijne[0..x].into_iter().all(|t| t < tree) || lijne[x+1..].into_iter().all(|t| t < tree) {
                return true
            }
        }
        let row : Vec<_> = self.trees.iter().map(|p| {
            let value_at_row = p.get(x).unwrap().clone();
            return value_at_row
        }).collect();


        if row[0..y].into_iter().all(|t| t < tree) || row[y+1..].into_iter().all(|t| t < tree) {
            return true
        }
        return false
    }

    fn get_scenic_score(&self, (x, y): (usize, usize)) -> i32 {
        let tree : &u32 = self.trees.get(y).unwrap().get(x).unwrap();

        let mut right = 0;
        let mut left = 0;
        let mut top = 0;
        let mut bottom = 0;
        //check horizontal
        for line in self.trees.get(y) {
            for lower in line[0..x].iter().rev() {
                right += 1;
                if lower >= tree {
                    break;
                }

            }
            for heiger in &line[x+1..] {
                left += 1;
                if heiger >= tree {
                    break;
                }

            }
        }
        let row : Vec<_> = self.trees.iter().map(|p| {
            let value_at_row = p.get(x).unwrap().clone();
            return value_at_row
        }).collect();

        for lower in row[0..y].iter().rev() {
            top += 1;
            if lower >= tree {
                break;
            }
        }
        for heiger in &row[y+1..] {
            bottom += 1;
            if heiger >= tree {
                break;
            }
        }

        return right * left * top * bottom;
    }
}