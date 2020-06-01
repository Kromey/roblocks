mod command;

pub use command::Command;
use std::io;

pub struct Robot {
    table: Vec<Vec<usize>>,
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

        let table: Vec<Vec<usize>> = (0..table_size)
            .map(|i| vec![i])
            .collect();

        Robot { table }.main_loop(&mut buf)
    }

    fn main_loop(&self, buf: &mut impl io::BufRead) -> Result<(), io::Error> {
        let mut input = String::with_capacity(20);

        loop {
            input.clear();
            buf.read_line(&mut input)?;

            let cmd = match input.to_lowercase().trim() {
                "" => continue,
                "quit" => Command::Quit,
                "print" => Command::PrintTable,
                cmd => Command::from(cmd),
            };

            println!("Command: {:?}", cmd);

            if let Command::Quit = cmd {
                break;
            }
        };

        Ok(())
    }
}
