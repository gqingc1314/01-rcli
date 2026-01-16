mod cli;
mod process;

pub use cli::{
    Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand, GenPassOpts, Opts,
    OutputFormat, SubCommandCmd,
};
pub use process::*;
