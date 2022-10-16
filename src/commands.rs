use clap::Parser;
use std::path::PathBuf;

// Simple program to greet a person

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}

// ------------------------------------------------------------
//
// use clap::{Parser, Subcommand};
// use std::path::PathBuf;
//
// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]
// struct Cli {
//     /// Optional name to operate on
//     name: Option<String>,
//
//     /// Sets a custom config file
//     #[arg(short, long, value_name = "FILE")]
//     config: Option<PathBuf>,
//
//     /// Turn debugging information on
//     #[arg(short, long, action = clap::ArgAction::Count)]
//     debug: u8,
//
//     #[command(subcommand)]
//     command: Option<Commands>,
// }
//
// #[derive(Subcommand)]
// enum Commands {
//     /// does testing things
//     Test {
//         /// lists test values
//         #[arg(short, long)]
//         list: bool,
//     },
// }
