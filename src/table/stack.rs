/// This object handles finishing a move Command
pub struct Stack<'a> {
    pub table: &'a mut Vec<Vec<usize>>,
    pub from_slot: usize,
    pub from_idx: usize,
    pub move_pile: bool,
}

impl Stack<'_> {
    /// Move the block(s) to the top of the stack containing the block `to`
    pub fn over(mut self, to: usize) {
        // Find the index of the stack we're moving to
        if let Some((to_slot, _)) = self.table
            .iter()
            .enumerate()
            .find(|(_, pile)| {
                pile.iter().any(|&block| block == to)
            })
        {
            // Make sure we're not moving to the same stack!
            if to_slot != self.from_slot {
                let mut moving = self.get_moving_pile();

                // Simply take the pile we're moving and append it to this stack
                self.table[to_slot].append(&mut moving);
            }
        }
    }

    /// Move our block(s) directly onto block `to`
    ///
    /// We first move any blocks already on top of `to` back to their original slots, then pile our
    /// block(s) on top of it.
    pub fn onto(mut self, to: usize) {
        if let Some((to_slot, _)) = self.table
            .iter()
            .enumerate()
            .find(|(_, pile)| {
                pile.iter().any(|&block| block == to)
            })
        {
            if to_slot != self.from_slot {
                // Get the pile we're moving
                let mut moving = self.get_moving_pile();
                // Temporary holding space for any blocks we have to return
                let mut returning = vec![];

                loop {
                    if to == *self.table[to_slot].last().unwrap() {
                        // Don't do anything to the block we're looking for, just stop taking
                        // blocks from this pile
                        break;
                    } else {
                        // Take the top block off this pile and save it for returning later
                        returning.push(self.table[to_slot].pop().unwrap());
                    }
                }

                // Return any blocks we took off the pile
                self.return_blocks(returning);
                // Now, finally, we can stack our block(s) atop our destination block
                self.table[to_slot].append(&mut moving);
            }
        }
    }

    /// Return all blocks in `blocks` to their original locations
    fn return_blocks(&mut self, mut blocks: Vec<usize>) {
        // Draining the vector of blocks
        for block in blocks.drain(..) {
            // Put each block on the stack whose slot matches its number
            self.table[block].push(block);
        }
    }

    /// Get the pile we're moving
    ///
    /// If we're moving just one block, this returns "pile" with a size of 1
    fn get_moving_pile(&mut self) -> Vec<usize> {
        if !self.move_pile {
            // If we're only moving one block, we need to first return any blocks on top of it
            let drain_from = self.from_idx + 1;
            // `drain()` will panic if we try to drain blocks that aren't there (e.g. if the block
            // we're after is the top one on the stack), so we need to guard against that here
            if drain_from < self.table[self.from_slot].len() {
                // Drain each block into a temporary vec
                let blocks: Vec<usize> = self.table[self.from_slot].drain(drain_from..).collect();
                // Now return those blocks
                self.return_blocks(blocks);
            }
        }

        // Grab the pile starting from our target block
        self.table[self.from_slot].split_off(self.from_idx)
    }
}
