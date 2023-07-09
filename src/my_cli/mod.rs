// use clap::Parser;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
pub mod foo;
pub mod handler;
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
    #[arg(short, long, value_enum)]
    //default_value = "TimeDomain::CurrentMonth"
    time_domain: TimeDomain,
}
// #[derive(Debug, Args, Clone, Copy)]
// // #[command(args_conflicts_with_subcommands = true)]
// pub struct MyData {
//     // #[arg(value_enum)]
//     timeDomain: TimeDomain,
// }
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum TimeDomain {
    CurrentMonth,
    PresentMonth,
    Future,
}
