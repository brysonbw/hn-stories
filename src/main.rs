//! > **A command line interface to browse and open Hacker News stories from the terminal.**
//!
//! ## Install
//! ```console
//! $ cargo install hn-stories
//! ```
//!
//! ## Example
//!```console
//! $ hn-stories -s t -l 10
//! ```
//!
//! ## Usage
//! ```console
//! $ hn-stories <OPTIONS>
//! ```
//!
//! Options:
//!   
//!   -s, --story
//!           Story type (top, new, best, ask, show, job)
//!
//!   -l, --limit
//!           The number of stories to fetch and display in the terminal UI
//!
//!   -h, --help
//!           Print help (see a summary with '-h')
//!
//!   -V, --version
//!           Print version
//!

mod api;
mod args;
mod client;
mod models;
mod types;
mod ui;
mod utils;

use std::process;

use chroma_print::{print_error, print_info};
use clap::{Error, Parser, error::ErrorKind};

use crate::{args::Args, client::HackerNewsClient, ui::terminal::TerminalUserInterface};

#[tokio::main]
async fn main() {
    let args = Args::try_parse().unwrap_or_else(|error: Error| match error.kind() {
        ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => {
            print_info!("{}", error);
            process::exit(0);
        }
        _ => {
            print_error!("Error: {}", error);
            process::exit(1);
        }
    });

    let gateway = HackerNewsClient::new(None);
    let ui = TerminalUserInterface;

    args.run(gateway, ui).await.unwrap_or_else(|error| {
        print_error!("Error: {error}");
        process::exit(1);
    });
}
