use std::io;

#[derive(Debug)]
enum TargetSpec {
    Block(usize),
    Pile(usize),
}

#[derive(Debug)]
pub struct Command {
    item: TargetSpec,
    dest: TargetSpec,
}

impl Command {
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
    /// use roblocks::Command;
    ///
    /// let stdin = stdin();
    /// let mut locked = stdin.lock();
    ///
    /// Command::read(&mut locked);
    /// ```
    ///
    /// Read from a file:
    ///
    /// ```no_run
    /// use std::io::BufReader;
    /// use std::fs::File;
    /// use roblocks::Command;
    ///
    /// let f = File::open("commands.txt").unwrap();
    /// let mut file_buf = BufReader::new(f);
    ///
    /// Command::read(&mut file_buf);
    /// ```
    pub fn read(buf: &mut impl io::BufRead) -> Result<(), io::Error> {
        let mut input = String::new();

        loop {
            input.clear();
            buf.read_line(&mut input)?;

            match input.trim() {
                "" => continue,
                "quit" => break,
                cmd => println!("{:?}", Command::from(cmd)),
            };
        };

        Ok(())
    }
}

impl From<&str> for Command {
    fn from(s: &str) -> Command {
        let cmd: Vec<&str> = s
            .trim()
            .split_whitespace()
            .collect();

        let item = match cmd[0] {
            "move" => TargetSpec::Block(cmd[1].parse().unwrap()),
            "pile" => TargetSpec::Pile(cmd[1].parse().unwrap()),
            _ => panic!("Bad input!"),
        };

        let dest = match cmd[2] {
            "onto" => TargetSpec::Block(cmd[3].parse().unwrap()),
            "over" => TargetSpec::Pile(cmd[3].parse().unwrap()),
            _ => panic!("Bad input!"),
        };

        Command {
            item,
            dest,
        }
    }
}
