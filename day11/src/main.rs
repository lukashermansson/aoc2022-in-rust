extern crate core;


use std::collections::VecDeque;
use ethnum::internal::uint;

fn main() {

    let mut monkeys = vec![
        Monkey {
            touched: 0,
            items: VecDeque::from_iter([54, 98, 50, 94, 69, 62, 53, 85]),
            operation: Box::new(|i| i * 13),
            test: 3,
            true_receiver: 2,
            false_receiver: 1
        },
        Monkey {
            touched: 0,
            items: VecDeque::from_iter([71, 55, 82]),
            operation: Box::new(|i| i + 2),
            test: 13,
            true_receiver: 7,
            false_receiver: 2
        },
        Monkey {
            touched: 0,
            items: VecDeque::from_iter([77, 73, 86, 72, 87]),
            operation: Box::new(|i| i + 8),
            test: 19,
            true_receiver: 4,
            false_receiver: 7
        },
        Monkey {
            touched: 0,
            items: VecDeque::from_iter([97, 91]),
            operation: Box::new(|i| i + 1),
            test: 17,
            true_receiver: 6,
            false_receiver: 5
        },
        Monkey {
            touched: 0,
            items: VecDeque::from_iter([78, 97, 51, 85, 66, 63, 62]),
            operation: Box::new(|i| i * 17),
            test: 5,
            true_receiver: 6,
            false_receiver: 3
        },
        Monkey {
            touched: 0,
            items: VecDeque::from_iter([88]),
            operation: Box::new(|i| i + 3),
            test: 7,
            true_receiver: 1,
            false_receiver: 0
        },
        Monkey {
            touched: 0,
            items: VecDeque::from_iter([87 , 57, 63, 86, 87, 53]),
            operation: Box::new(|i| i * i),
            test: 11,
            true_receiver: 5,
            false_receiver: 0
        },
        Monkey {
            touched: 0,
            items: VecDeque::from_iter([73, 59, 82, 65]),
            operation: Box::new(|i| i + 6),
            test: 2,
            true_receiver: 4,
            false_receiver: 3
        }
    ];


    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            for _item_index in 0..monkeys[m].items.len() {
                let monkey = monkeys.get_mut(m).unwrap();

                let Some(item) = monkey.inspect() else { continue; };
                let recipiant = monkey.test(item);

                let reciver = monkeys.get_mut(recipiant).unwrap();
                reciver.items.push_back(item);
            }
        }
    }

    let mut tocuhed: Vec<i128> = monkeys.iter().map(|x| x.touched).collect();

    tocuhed.sort_by(|x, y| y.cmp(x));

    let mut iter = tocuhed.iter();
    let (Some(x), Some(y)) = (iter.next(), iter.next()) else { panic!("Needs to be two values") };

    println!("Result: {}", x * y);

}
const LOWEST_COMMON: i128 = 9699690;

struct Monkey {
    touched: i128,
    items: VecDeque<i128>,
    operation: Box<dyn Fn(i128) -> i128>,
    test: i128,
    true_receiver: usize,
    false_receiver: usize
}

impl Monkey {
    fn inspect(&mut self) -> Option<i128> {
        let item = self.items.pop_front()?;

        self.touched += 1;

        let item_to_return = (self.operation)(item);
        return Some(item_to_return % LOWEST_COMMON)
    }

    fn test(&self, item: i128) -> usize {

        if item % self.test == 0 {
            return self.true_receiver
        }else {
            return self.false_receiver
        }
    }
}