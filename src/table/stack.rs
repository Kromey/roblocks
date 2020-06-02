pub struct Stack<'a> {
    pub table: &'a mut Vec<Vec<usize>>,
    pub from_slot: usize,
    pub from_range: (usize, usize),
}

impl Stack<'_> {
    pub fn over(self, to: usize) {
        if let Some((to_slot, _)) = self.table
            .iter()
            .enumerate()
            .find(|(_, pile)| {
                pile.iter().any(|&block| block == to)
            })
        {
            if to_slot != self.from_slot {
                println!("Moving from slot {} to slot {}", self.from_slot, to_slot);
                let mut moving = self.table[self.from_slot].split_off(self.from_range.0);

                self.table[to_slot].append(&mut moving);
            }
        }
    }
}
