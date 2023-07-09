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
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    Free(FreeQuestion),
    Analyze(AnalyzeQuestion),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct FreeQuestion {
    #[arg(short, long)]
    question: String,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct AnalyzeQuestion {
    #[arg(short, long)]
    question: String,
    #[clap(subcommand)]
    data: MyData,
}
#[derive(Debug, Args, Clone, Copy)]
pub struct MyData {
    #[arg(short, long)]
    timeDomain: TimeDomain,
}
#[derive(Debug, ValueEnum, Copy, Clone)]
pub enum TimeDomain {
    Past,
    Present,
    Future,
}
