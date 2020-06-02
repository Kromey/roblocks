mod stack;

use stack::Stack;

/// A struct to represent out table of blocks
pub struct Table {
    table: Vec<Vec<usize>>,
}

impl Table {
    /// Create a new Table of size `size`
    ///
    /// The table is initialized with `size` stacks of 1 block, where each block is represented as
    /// a number corresponding to the  index of its initial stack.
    pub fn new(size: usize) -> Table {
        assert!(size > 0);

        let table: Vec<Vec<usize>> = (0..size)
            .map(|i| vec![i])
            .collect();

        Table { table }
    }

    /// Print the table to standard output
    ///
    /// The table is printed with each "slot" on the table representing by its index followed by a
    /// colon. If there are any blocks in that slot, the stack is represented as a list of block
    /// numbers each separated by a space.
    pub fn print(&self) {
        self.table.iter().enumerate().for_each(|(i, pile)| {
            // Use print!() so we can continue to print on the same line
            print!("{}:", i);
            if pile.len() > 0 {
                // For each block in this slot, print a space followed by the block number
                pile.iter().for_each(|block| print!(" {}", block));
            };
            println!("");
        });
    }

    /// Move the pile containing the block numbered `from`
    pub fn pile(&mut self, from: usize) -> Stack {
        if let Some((slot, block_idx)) = self.find_pile(from) {
            Stack {
                table: &mut self.table,
                from_slot: slot,
                from_idx: block_idx,
                move_pile: true,
            }
        } else {
            panic!("Block not found: {}", from);
        }
    }

    /// Move the block numbered `from`
    pub fn block(&mut self, from: usize) -> Stack {
        if let Some((slot, block_idx)) = self.find_pile(from) {
            Stack {
                table: &mut self.table,
                from_slot: slot,
                from_idx: block_idx,
                move_pile: false,
            }
        } else {
            panic!("Block not found: {}", from);
        }
    }

    /// Find the pile on the table containing the block `target`
    ///
    /// We return a tuple of the form (slot, idx), where:
    ///  * `slot` is the index of the pile we're moving from
    ///  * `idx` is the index in that pile of the block we're moving
    fn find_pile(&self, target: usize) -> Option<(usize, usize)> {
        self.table
            .iter()
            .enumerate()
            .find_map(|(slot, pile)| {
                let (idx,_) = pile
                    .iter()
                    .enumerate()
                    .find(|(_, &block)| block == target)?;

                Some((slot, idx))
            })
    }
}
