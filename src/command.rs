#[derive(Debug)]
pub enum Target {
    Block(usize),
    Pile(usize),
}

#[derive(Debug)]
pub enum Command {
    Continue,
    Quit,
    Move(Target,Target),
    PrintTable,
}

impl Command {
    fn move_command(s: &str) -> Command {
        let cmd: Vec<&str> = s
            .trim()
            .split_whitespace()
            .collect();

        let item = match cmd[0] {
            "move" => Target::Block(cmd[1].parse().unwrap()),
            "pile" => Target::Pile(cmd[1].parse().unwrap()),
            _ => panic!("Bad input!"),
        };

        let dest = match cmd[2] {
            "onto" => Target::Block(cmd[3].parse().unwrap()),
            "over" => Target::Pile(cmd[3].parse().unwrap()),
            _ => panic!("Bad input!"),
        };

        Command::Move(item, dest)
    }
}

impl From<&String> for Command {
    fn from(s: &String) -> Command {
        match s.to_lowercase().trim() {
            "" => Command::Continue,
            "quit" => Command::Quit,
            "print" => Command::PrintTable,
            move_cmd => Command::move_command(move_cmd),
        }
    }
}
