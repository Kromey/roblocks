pub struct Stack<'a> {
    pub table: &'a mut Vec<Vec<usize>>,
    pub from_slot: usize,
    pub from_idx: usize,
    pub move_pile: bool,
}

impl Stack<'_> {
    pub fn over(mut self, to: usize) {
        if let Some((to_slot, _)) = self.table
            .iter()
            .enumerate()
            .find(|(_, pile)| {
                pile.iter().any(|&block| block == to)
            })
        {
            if to_slot != self.from_slot {
                println!("Moving from slot {} to slot {}", self.from_slot, to_slot);
                let mut moving = self.get_moving_pile();

                self.table[to_slot].append(&mut moving);
            }
        }
    }

    pub fn onto(mut self, to: usize) {
        if let Some((to_slot, _)) = self.table
            .iter()
            .enumerate()
            .find(|(_, pile)| {
                pile.iter().any(|&block| block == to)
            })
        {
            if to_slot != self.from_slot {
                println!("Moving from slot {} to slot {}", self.from_slot, to_slot);
                let mut moving = self.get_moving_pile();
                let mut returning = vec![];

                loop {
                    if to == *self.table[to_slot].last().unwrap() {
                        break;
                    } else {
                        returning.push(self.table[to_slot].pop().unwrap());
                    }
                }

                self.return_blocks(returning);
                self.table[to_slot].append(&mut moving);
            }
        }
    }

    fn return_blocks(&mut self, mut blocks: Vec<usize>) {
        for block in blocks.drain(..) {
            self.table[block].push(block);
        }
    }

    fn get_moving_pile(&mut self) -> Vec<usize> {
        if !self.move_pile {
            let drain_from = self.from_idx + 1;
            if drain_from < self.table[self.from_slot].len() {
                let blocks: Vec<usize> = self.table[self.from_slot].drain(drain_from..).collect();
                self.return_blocks(blocks);
            }
        }

        self.table[self.from_slot].split_off(self.from_idx)
    }
}
