pub type BinResult<T> = std::result::Result<T, BinError>;

#[derive(Debug)]
pub enum BinError {
    NoArgument,
    HelpArgument,
    InvalidInput(String),
}
