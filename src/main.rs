#![allow(warnings)]
use core::panic;
use std::path::PathBuf;
use std::fs;

use clap::{Parser, CommandFactory, error::ErrorKind};
use cli::PhoenixCli;
mod cli;

fn main() {
    let cli = PhoenixCli::parse();

    match cli.subcmd {
        cli::Commands::Compile { scan, project } => compile(scan, project),
        cli::Commands::Run { scan, compiled, debug, file } => todo!(),
    }
}

fn compile(scan: bool, file: PathBuf) {
    if let Some(ext) = file.extension() {
        if ext != "phx" { 
            PhoenixCli::command().error(
                ErrorKind::InvalidValue, "File must be either .phx").exit()
        }
    } else { PhoenixCli::command().error(
            ErrorKind::InvalidValue, "File must be either .phx").exit() }

    let src =  fs::read_to_string(file).map_err(|err| panic!("{}", err.to_string())).unwrap();
}
