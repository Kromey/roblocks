mod error;

pub use error::CommandError;
use std::convert::TryFrom;

type CommandResult = std::result::Result<Command, CommandError>;

/// Target specifies whether we're manipulating a single block or a whole pile
#[derive(Debug)]
pub enum Target {
    Block(usize),
    Pile(usize),
}

/// Command defines which operations we can be commanded to perform
#[derive(Debug)]
pub enum Command {
    /// A no-op
    Continue,
    /// Print the table and terminate the robot
    Quit,
    /// Move a Target to another pile
    Move(Target,Target),
    /// Print the table
    PrintTable,
}

impl Command {
    /// Process a line of input into a Move Command
    ///
    /// Commands take the form `<from_verb> <block> <to_verb> <block>`, where:
    ///  * <from_verb> can be `move` for a single block, or `pile` for the pile of blocks
    ///  * <to_verb> can be `onto` for a single block, of `over` for the pile of blocks
    ///  * <block> is the block number
    fn move_command(s: &str) -> CommandResult {
        // Split our command into individual tokens
        let cmd: Vec<&str> = s
            .split_whitespace()
            .collect();

        // Valid Commands are exactly 4 words, anything else is invalid
        if cmd.len() != 4 {
            return Err(CommandError::BadCommand(s.into()));
        }

        // Parse our 2 block identifies into integers
        let item_id = cmd[1].parse()?;
        let dest_id = cmd[3].parse()?;

        // Make sure we're not moving a block to itself
        if item_id == dest_id {
            return Err(CommandError::ImpossibleMove);
        }

        // Identify which type of move we're performing
        let item = match cmd[0] {
            "move" => Target::Block(item_id),
            "pile" => Target::Pile(item_id),
            s => return Err(CommandError::BadCommand(s.into())),
        };

        // Identify what we're moving to
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

    /// This trait allows us the flexibility to not be infallible
    ///
    /// In order to provide feedback to the user, we return errors on invalid input. See
    /// `CommandError` for which errors we may return.
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
