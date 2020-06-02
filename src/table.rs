mod stack;

use stack::Stack;

pub struct Table {
    table: Vec<Vec<usize>>,
}

impl Table {
    pub fn new(size: usize) -> Table {
        assert!(size > 0);

        let table: Vec<Vec<usize>> = (0..size)
            .map(|i| vec![i])
            .collect();

        Table { table }
    }

    pub fn print(&self) {
        self.table.iter().enumerate().for_each(|(i, pile)| {
            print!("{}:", i);
            if pile.len() > 0 {
                pile.iter().for_each(|block| print!(" {}", block));
            };
            println!("");
        });
    }

    pub fn pile(&mut self, from: usize) -> Stack {
        if let Some((i, pile)) = self.find_pile(from) {
            let moving = pile.split_off(i);
            println!("Moving {:?}; leaving behind {:?}", moving, pile);

            Stack {
                table: &mut self.table,
                stack: moving,
            }
        } else {
            panic!("Block not found: {}", from);
        }
    }

    fn find_pile(&mut self, target: usize) -> Option<(usize, &mut Vec<usize>)> {
        self.table
            .iter_mut()
            .find_map(|pile| {
                let (i,_) = pile
                    .iter()
                    .enumerate()
                    .find(|(_, &block)| block == target)?;

                Some((i, pile))
            })
    }
}
