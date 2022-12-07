use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::{io, vec};
use std::io::BufRead;
use std::str::SplitWhitespace;

fn main() {
    let file = File::open("./input.txt").unwrap();

    let lines : Vec<_> = io::BufReader::new(file).lines().collect();


    let mut root = Directory {
        folders: HashMap::new(),
        files: vec![],
    };
    let mut cursor = &mut root;


    let mut location: Vec<String> = vec![];


    for l in lines {
        let l = l.unwrap();
        let mut parts = l.split_whitespace();

        let begin = parts.next().unwrap();
        match begin {
            "$" => {
                //command
                let command = parts.next().unwrap();
                if command == "cd" {
                    let folder = parts.next().unwrap();
                    if folder == ".." {
                        location.pop();

                        let mut c = &mut root;
                        for part in location.iter() {
                            c = c.folders.get_mut(&*part).unwrap();
                        }
                        cursor = c;
                    }else {
                        if folder != "/" {
                            let mut dir = Directory {
                                folders: HashMap::new(),
                                files: vec![],
                            };
                            cursor.folders.insert(folder.to_string(), dir);
                            cursor = cursor.folders.get_mut(&*folder.to_string()).unwrap();
                            location.push(folder.to_string());
                        }
                    }
                }
            },
            "dir" => {
                //is a dir, ignore
            },
            x => {
                let size = x.parse::<i128>().unwrap();
                let name = parts.next().unwrap().to_string();

                cursor.files.push(SystemFile {
                    name,
                    size,
                })
            }
        }
    }
    let x = get_folder_size(&root);
    let remaining = 70000000 - x;
    let neededSpace = 30000000 - remaining;
    let mut map = vec![];
    get_subfolder_sizes("", &root, &mut map);
    let mut map: Vec<_>= map.into_iter().collect();

    let mut total: Vec<_> = map.into_iter().filter(|x| x.1 > neededSpace).collect();

    total.sort_by(|x, y| x.1.cmp(&y.1));

    println!("{:#?}", total);
    println!("{:#?}", x);
    println!("{:#?}", remaining);
    println!("{:#?}", neededSpace);

}

fn get_subfolder_sizes(current: &str, dir: &Directory, map : &mut Vec<(String, i64)>) {
    for i in &dir.folders {
        get_subfolder_sizes(&(current.to_string() + "/" + i.0), i.1, map);

    }
    map.push((current.to_string(), get_folder_size(dir)));
}
fn get_folder_size(dir : &Directory) -> i64 {
    let mut total = 0;

    for dir in dir.folders.values() {
        total += get_folder_size(dir)
    }

    for files in &dir.files {
        total += files.size as i64;
    }
    return total;
}


#[derive(Debug)]
struct Directory {
    folders: HashMap<String, Directory>,
    files: Vec<SystemFile>
}
#[derive(Debug)]
struct SystemFile {
    name: String,
    size: i128
}