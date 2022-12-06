use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./input.txt").unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).expect("Should be utf-8");

    let chars = content.chars();

    let mut prev4 =  VecDeque::from([]);
    const SAMPLE_LENGHT: usize = 4;
    let ans  = chars.enumerate().map(|(index, char)| {
        if index > SAMPLE_LENGHT {
            if filter_uniq(prev4.clone()).len() == prev4.len() {
                return Some(index)
            }
        }

        prev4.push_back(char);
        if prev4.len() > SAMPLE_LENGHT {
            prev4.pop_front();
        }
        return None
    }).filter(|x| x.is_some()).next().unwrap();

    println!("{}", ans.unwrap())
}

//to figure out the uniqe elements, we make it a hash map and turn it back
fn filter_uniq(vec: VecDeque<char>) -> VecDeque<char> {
    vec.into_iter()
        .map(|course| course)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}