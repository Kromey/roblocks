mod command;
mod table;

use command::{Command, CommandError};
use std::convert::TryFrom;
use std::io;
use table::Table;

pub struct Robot {
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
        let mut setup = String::new();
        buf.read_line(&mut setup)?;

        let table_size: usize = setup.trim().parse().unwrap();
        println!("Table size: {}", table_size);

        Robot { table: Table::new(table_size) }.main_loop(&mut buf)
    }

    fn main_loop(&mut self, buf: &mut impl io::BufRead) -> Result<(), io::Error> {
        let mut input = String::with_capacity(20);

        loop {
            input.clear();
            buf.read_line(&mut input)?;

            match Command::try_from(&input) {
                Ok(cmd) => {
                    println!("Command: {:?}", cmd);

                    match cmd {
                        Command::Continue => continue,
                        Command::Quit => {
                            self.print_table();
                            break;
                        },
                        Command::PrintTable => self.print_table(),
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

    fn print_table(&self) {
        self.table.print();
    }

    fn handle_move(&mut self, from: command::Target, to: command::Target) {
        match (from, to) {
            (command::Target::Pile(from), command::Target::Pile(to)) => {
                self.table.pile(from).over(to);
            },
            (from, to) => {
                println!("{:?}, {:?}", from, to);
            },
        };
    }
}
