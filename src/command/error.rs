use std::num::ParseIntError;

/// Types of errors we might encounter while parsing Command input
#[derive(Debug)]
pub enum CommandError {
    /// An invalid block identifier was supplied
    BadBlockId(ParseIntError),
    /// An invalid Command string was supplied
    BadCommand(String),
    /// Tried to move a block to itself
    ImpossibleMove,
}

impl From<ParseIntError> for CommandError {
    fn from(e: ParseIntError) -> CommandError {
        CommandError::BadBlockId(e)
    }
}
