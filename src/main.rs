extern crate clap;

use std::path::PathBuf;
use std::process::exit;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short,long,value_parser)]
    config: Option<PathBuf>,
}

fn main() {
    // Load args
    let args = Args::parse();\

    // Validate config path
    if let Some(config_path) = args.config.as_deref() {
        // TODO: find git repos
        // TODO: loop over list of repos to perform pull & push
    } else {
        exit(1);
    }
}