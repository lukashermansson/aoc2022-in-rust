use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let file = File::open("./input.txt").unwrap();

    let lines : Vec<_> = io::BufReader::new(file).lines().collect();

    let mut count = 0;

    for l in lines {
        let l = l.unwrap();
        let (first, second)  = l.split_once(",").unwrap();

        let (x, y) = first.split_once("-").unwrap();
        let a = x.parse::<i32>().unwrap()..=y.parse::<i32>().unwrap();

        let (x, y) = second.split_once("-").unwrap();
        let b = x.parse::<i32>().unwrap()..=y.parse::<i32>().unwrap();


        let result = b.clone().map(|x| a.contains(&x)).any(|x| x == true) || a.map(|x| b.contains(&x)).any(|x| x == true);

        if(result) {
            count += 1;
        }
    }
    println!("{}", count)
}
