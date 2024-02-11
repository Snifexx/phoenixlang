use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct PhoenixCli {
    #[command(subcommand)]
    pub subcmd: Commands,

}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Compiles phx source file to executable flms file
    Compile {
        /// Whether to print scanned input or not
        #[arg(short, long)]
        scan: bool,
        
        /// Must be a directory with a Feather.toml
        project: PathBuf,
    },

    /// Execute either compiled flms file or compiles and runs a phx file
    Run {
        /// Whether to print scanned input or not (only works for .phx files)
        #[arg(short, long)]
        scan: bool,

        /// Whether to print result of the compiler or not (only works for .phx files)
        #[arg(short, long)]
        compiled: bool,

        /// Whether to debug each instruction executed by the vm 
        #[arg(short, long)]
        debug: bool,

        /// Can be a .flms or a directory with a Feather.toml
        file: PathBuf,
    }
}
