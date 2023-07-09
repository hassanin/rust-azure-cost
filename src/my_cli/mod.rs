// use clap::Parser;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
pub mod foo;
pub fn hello() {
    println!("Hello, world!");
}

#[derive(Debug, Parser)]
pub struct Cli{
    #[clap(subcommand)]
    command:Commands
}
#[derive(Debug, Subcommand)]
enum Commands{
  Free(FreeQuestion)
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct FreeQuestion{
    #[arg(short, long)]
    question: String,
}