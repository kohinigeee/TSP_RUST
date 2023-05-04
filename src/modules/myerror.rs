use anyhow::{self, Context};
use thiserror::{Error};

#[derive(Debug,Error)]
pub enum Myerror {
    #[error("Input Error [file path : {}] [message : {}]", .fpath, .message)]
    MyInputFileError {fpath: String, message: String},
}