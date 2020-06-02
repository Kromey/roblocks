pub struct Stack<'a> {
    pub table: &'a mut Vec<Vec<usize>>,
    pub stack: Vec<usize>,
}

impl Stack<'_> {
    pub fn over(self, to: usize) {
        if let Some(pile) = self.table
            .iter_mut()
            .find(|pile| {
                pile.iter().any(|&block| block == to)
            })
        {
            pile.extend(self.stack);
        }
    }
}
