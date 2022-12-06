use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./input.txt").unwrap();

    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();

    let (initial_state, moves) = buffer.split_once("\n\n").unwrap();

    let mut storage = parse_storage(initial_state);

    for currentMove in moves.lines()
        .map(|x| parse_move(x).unwrap()) {
        storage.execute_move_forklift(currentMove);
    }

    println!("state after all moves:\n{}", storage);
    println!("top crates: {}", storage.get_top_crates().into_iter()
        .filter(|x| x.is_some()).map(|x| x.unwrap()).collect::<String>())
}

fn parse_move(input: &str) -> Option<ForkliftAction> {
    let mut parts = input.split_whitespace().map(|s| s.parse::<u8>());

    match (parts.next(), parts.next(), parts.next(), parts.next(), parts.next(), parts.next()) {
        (Some(Err(_)), Some(Ok(amount)), Some(Err(_)), Some(Ok(from)), Some(Err(_)), Some(Ok(to))) => {
            Some(ForkliftAction {
                from,
                to,
                amount,
            })
        },
        _ => None
    }
}


fn parse_storage(input: &str) -> StorageFacility {
    let mut lines: Vec<&str> = input.lines().collect();
    lines.reverse();

    let mut storage = vec![];
    let mut index = 1;
    let mut lines = lines.into_iter();
    let max_index = lines.next().unwrap().len();
    while index < max_index {
        let mut crates = vec![];
        for l in lines.clone() {
            let chars = l.chars().nth(index).unwrap();
            if chars != ' ' {
                crates.push(chars);
            }
        }
        storage.push(crates);
        index += 4
    }

    return StorageFacility {
        piles: storage,
    }
}

struct StorageFacility {
    piles: Vec<Vec<char>>
}

struct ForkliftAction {
    from: u8,
    to: u8,
    amount: u8
}

impl StorageFacility {
    fn execute_move_forklift(&mut self, action: ForkliftAction) {
        let mut buffer = vec![];
        let line : &mut Vec<char> = &mut self.piles.get_mut((action.from - 1) as usize).unwrap();

        for _ in 0..action.amount {
            buffer.push(line.pop().unwrap())
        }
        buffer.reverse();

        let line = &mut self.piles.get_mut((action.to - 1) as usize).unwrap();

        for _ in 0..action.amount {
            line.push(buffer.pop().unwrap())
        }
    }
    fn execute_move_super_forklift(&mut self, action: ForkliftAction) {
        let mut buffer = vec![];
        let line : &mut Vec<char> = &mut self.piles.get_mut((action.from - 1) as usize).unwrap();

        for _ in 0..action.amount {
            buffer.push(line.pop().unwrap())
        }

        let line = &mut self.piles.get_mut((action.to - 1) as usize).unwrap();

        for _ in 0..action.amount {
            line.push(buffer.pop().unwrap())
        }
    }

    fn get_top_crates(&self) -> Vec<Option<&char>> {
        let mut crates = vec![];

        for pile in &self.piles {
            let top_crate = pile.get(pile.len() - 1);

            crates.push(top_crate)
        }

        return crates;
    }
}

impl Display for StorageFacility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.piles {
            for cr in line {
                f.write_str(&*format!(" [{}] ", cr)).unwrap();
            }
            f.write_str("\n").unwrap();
        }

        Ok(())
    }
}