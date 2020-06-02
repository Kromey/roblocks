//! Solving the Blocks Problem
//!
//! This crate implements one solution to the [Blocks Problem], which simulates programming a robotic
//! arm to manipulate a world represented as a series of blocks.
//!
//! [Blocks Problem]: https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=3&page=show_problem&problem=37
//!
//! A Robot will read commands from the supplied input buffer (see `Robot::run()` for details about
//! supplying the buffer) and execute them one by one.
//!
//! The first command must be a single integer specifying the size of the table. The Robot will
//! then be initialized with a table containing the specified number of blocks, each one in a
//! "stack" of exactly 1 block. Blocks are numbered from 0..n-1, where `n` is the size of the
//! table, and their initial locations are numbered likewise.
//!
//! During the Robot's operation, you can issue different types of commands to order the Robot to
//! manipulate the blocks on the table. The commands available to you are:
//!  1. "print": This prints the current state of the table.
//!  2. "quit": This prints the table and then terminates the Robot.
//!  3. "{verb} {block} {verb} {block}": Move the blocks on the table (see next section).
//!
//! Invalid commands will print an error message before the Robot resumes normal operation.
//!
//! # Moving blocks
//!
//! Blocks are moved by issuing a command in the form "{verb} {block} {verb} {block}".
//!
//! The first verb must be either `move` or `pile` and targets the first block, which must be an
//! integer corresponding to the block's number.
//!  * `move` will pick up the specified block after returning any blocks stacked above it to their
//!    initial stacks.
//!  * `pile` by contrast picks up the specified block and any blocks stacked on top of it.
//!
//! The second verb specifies how the destination is to be handled, identified by the second block
//! number in the command.
//!  * `onto` will first return any blocks above the destination block, then stack the moved
//!    block(s) on top of it.
//!  * `over` will simply stack to the moved block(s) on top of the stack.
mod command;
mod table;

use command::{Command, CommandError};
use std::convert::TryFrom;
use std::io;
use table::Table;

/// The Robot receives commands and manipulates the blocks on the table
pub struct Robot {
    /// The table of blocks
    table: Table,
}

impl Robot {
    /// Reads commands from the supplied buffer
    ///
    /// This will read until it reaches the command "quit".
    ///
    /// # Examples
    ///
    /// Read from standard input:
    ///
    /// ```no_run
    /// use std::io::stdin;
    /// use roblocks::Robot;
    ///
    /// let stdin = stdin();
    /// let mut locked = stdin.lock();
    ///
    /// Robot::run(&mut locked);
    /// ```
    ///
    /// Read from a file:
    ///
    /// ```no_run
    /// use std::io::BufReader;
    /// use std::fs::File;
    /// use roblocks::Robot;
    ///
    /// let f = File::open("commands.txt").unwrap();
    /// let mut file_buf = BufReader::new(f);
    ///
    /// Robot::run(&mut file_buf);
    /// ```
    pub fn run(mut buf: &mut impl io::BufRead) -> Result<(), io::Error> {
        // Read one line of setup input to determine the table size
        let mut setup = String::new();
        buf.read_line(&mut setup)?;

        let table_size: usize = setup.trim().parse().unwrap();

        // Create a Robot instance containing a Table of the specified size, and run it
        Robot { table: Table::new(table_size) }.main_loop(&mut buf)
    }

    /// The main program loop
    ///
    /// This loop runs until a "quit" command is received. On each iteration of the loop the Robot
    /// waits for a line of input, which it parses into a Command that it then executes.
    fn main_loop(&mut self, buf: &mut impl io::BufRead) -> Result<(), io::Error> {
        // Create a buffer with an initial capacity that should handle our largest commands
        // While it will still grow if/when necessary, this reduces unnecessary reallocations
        // during normal operation
        let mut input = String::with_capacity(20);

        loop {
            // Empty the buffer without touching its capacity
            input.clear();
            // Read a line from our input
            buf.read_line(&mut input)?;

            match Command::try_from(&input) {
                Ok(cmd) => {
                    // We've got a Command! Let's see what we need to do
                    match cmd {
                        // A catch-all for invalid commands, just start the loop over
                        Command::Continue => continue,
                        // Quit: Print the table and exit
                        Command::Quit => {
                            self.print_table();
                            break;
                        },
                        // Print: Print the table
                        Command::PrintTable => self.print_table(),
                        // A Move command; see `handle_move()` for the logic
                        Command::Move(from,to) => self.handle_move(from, to),
                    };
                },
                Err(err) => {
                    match err {
                        CommandError::BadBlockId(id) => eprintln!("Invalid block id: {}", id),
                        CommandError::BadCommand(cmd) => eprintln!("Invalid command: {}", cmd),
                        CommandError::ImpossibleMove => eprintln!("Cannot move a block onto/over itself"),
                    };
                },
            };
        };

        Ok(())
    }

    /// Print the table
    ///
    /// This delegates to `Table::print()`
    fn print_table(&self) {
        self.table.print();
    }

    /// Execute a move Command
    ///
    /// Given a from and to, each of which could be either a Pile or a Block, move the specified
    /// block(s):
    ///  * When moving a Pile, we simply pick up the target block and all blocks above it
    ///  * When moving a Block, any blocks above it are first returned to their initial slots
    ///  * When moving to a Pile, we just stack our block(s) on top
    ///  * When moving to a Block, we first return any blocks above the target before stacking
    fn handle_move(&mut self, from: command::Target, to: command::Target) {
        match (from, to) {
            (command::Target::Pile(from), command::Target::Pile(to)) => {
                self.table.pile(from).over(to);
            },
            (command::Target::Block(from), command::Target::Pile(to)) => {
                self.table.block(from).over(to);
            },
            (command::Target::Pile(from), command::Target::Block(to)) => {
                self.table.pile(from).onto(to);
            },
            (command::Target::Block(from), command::Target::Block(to)) => {
                self.table.block(from).onto(to);
            },
        };
    }
}
