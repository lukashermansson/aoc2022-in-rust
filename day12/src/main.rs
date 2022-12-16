use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::slice::SliceIndex;
use petgraph::algo::dijkstra;
use petgraph::dot::{Config, Dot};
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::DefaultIx;

fn main() {
    let file = File::open("./input.txt").unwrap();
    let vec: Vec<Vec<char>> = io::BufReader::new(file).lines().map(|x| x.unwrap().chars().collect()).collect();
    let mut graph = Graph::<_, ()>::new();

    let mut end = None;

    let mut starting_poses = vec![];

    for y in &vec {
        for x in y {
            let index = graph.add_node(x.clone());

            if x == &'S' {
                starting_poses.push(index)
            }
            if x == &'a' {
                starting_poses.push(index)
            }
            if x == &'E' {
                end = Some(index);
            }
        }
    }

    let mut edges: Vec<(DefaultIx, DefaultIx)> = vec![];
    let x_with = vec.get(0).unwrap().len();
    let y_width = vec.len();
    for y in 0..y_width {
        for x in 0..x_with {
            let current = &vec[y][x];
            if y > 0 {
                let top_y = y - 1;
                let other = &vec[top_y][x];
                if can_step_to(get_char_value(current), get_char_value(other)) {
                    edges.push(((y * x_with + x) as DefaultIx, (top_y * x_with + x) as DefaultIx));
                }
            }
            if y < y_width - 1 {
                let bottom = y + 1;
                let other = &vec[bottom][x];
                if can_step_to(get_char_value(current), get_char_value(other)) {
                    edges.push(((y * x_with + x) as DefaultIx, (bottom * x_with + x) as DefaultIx));
                }
            }
            if x < x_with - 1 {
                let right = x + 1;
                let other = &vec[y][right];
                if can_step_to(get_char_value(current), get_char_value(other)) {
                    edges.push(((y * x_with + x) as DefaultIx, (y * x_with + right) as DefaultIx));
                }
            }

            if x > 0 {
                let left = x - 1;
                let other = &vec[y][left];
                if can_step_to(get_char_value(current), get_char_value(other)) {
                    edges.push(((y * x_with + x) as DefaultIx, (y * x_with + left) as DefaultIx));
                }
            }
        }
    }

    graph.extend_with_edges(&edges);

    let paths = starting_poses.iter()
        .filter_map(|x| {
            let map = dijkstra(&graph, x.clone(), None, |_| 1);
            return map.get(&end.unwrap().into()).map(|c| c.clone());
        })
        .collect::<Vec<i32>>();

    println!("lenght: {:?}", paths.iter().min_by(|x, y| x.cmp(y)));
}

fn get_char_value(c: &char) -> i32 {
    let alpha = "abcdefghijklmnopqrstuvwxyz";

    if c == &'S' {
        return 0;
    }
    if c == &'E' {
        return 25;
    }

    return alpha.chars().position(|p| p == *c).unwrap() as i32;
}

fn can_step_to(this: i32, other: i32) -> bool {
    if this + 1 == other {
        return true;
    }
    if this == other {
        return true;
    }
    return if other < this {
        true
    } else {
        false
    };
}