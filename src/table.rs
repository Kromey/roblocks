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
        if let Some((slot, start, end)) = self.find_pile(from) {
            Stack {
                table: &mut self.table,
                from_slot: slot,
                from_range: (start, end),
            }
        } else {
            panic!("Block not found: {}", from);
        }
    }

    fn find_pile(&self, target: usize) -> Option<(usize, usize, usize)> {
        self.table
            .iter()
            .enumerate()
            .find_map(|(slot, pile)| {
                let (idx,_) = pile
                    .iter()
                    .enumerate()
                    .find(|(_, &block)| block == target)?;

                Some((slot, idx, pile.len()))
            })
    }
}
