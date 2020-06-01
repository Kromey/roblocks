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
    pub fn stdin() -> Result<(), io::Error> {
        let mut input = String::new();

        loop {
            input.clear();
            io::stdin().read_line(&mut input)?;

            match input.trim() {
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
