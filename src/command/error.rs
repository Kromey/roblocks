use std::num::ParseIntError;

#[derive(Debug)]
pub enum CommandError {
    BadBlockId(ParseIntError),
    BadCommand(String),
    ImpossibleMove,
}

impl From<ParseIntError> for CommandError {
    fn from(e: ParseIntError) -> CommandError {
        CommandError::BadBlockId(e)
    }
}
