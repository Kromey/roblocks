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

    fn get_moving_pile(&mut self) -> Vec<usize> {
        if self.move_pile {
            self.table[self.from_slot].split_off(self.from_idx)
        } else {
            vec![]
        }
    }
}
