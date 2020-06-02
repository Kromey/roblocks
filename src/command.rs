mod error;

pub use error::CommandError;
use std::convert::TryFrom;

type CommandResult = std::result::Result<Command, CommandError>;

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
    fn move_command(s: &str) -> CommandResult {
        let cmd: Vec<&str> = s
            .split_whitespace()
            .collect();

        let item_id = cmd[1].parse()?;
        let dest_id = cmd[3].parse()?;

        if item_id == dest_id {
            return Err(CommandError::ImpossibleMove);
        }

        let item = match cmd[0] {
            "move" => Target::Block(item_id),
            "pile" => Target::Pile(item_id),
            s => return Err(CommandError::BadCommand(s.into())),
        };

        let dest = match cmd[2] {
            "onto" => Target::Block(dest_id),
            "over" => Target::Pile(dest_id),
            s => return Err(CommandError::BadCommand(s.into())),
        };

        Ok(Command::Move(item, dest))
    }
}

impl TryFrom<&String> for Command {
    type Error = CommandError;

    fn try_from(s: &String) -> CommandResult {
        let cmd = match s.to_lowercase().trim() {
            "" => Command::Continue,
            "quit" => Command::Quit,
            "print" => Command::PrintTable,
            move_cmd => Command::move_command(move_cmd)?,
        };

        Ok(cmd)
    }
}
