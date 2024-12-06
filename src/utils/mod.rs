#[macro_use]
pub mod macros;

pub mod coord;
pub mod direction;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid command usage: {0}")]
    ClapError(#[from] clap::Error),
    #[error("Invalid input.")]
    InvalidInput,
    #[error("Unsupported day.")]
    UnsupportedDay,
}
