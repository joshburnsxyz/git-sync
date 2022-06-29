extern crate clap;

use std::path::PathBuf;
use std::process::exit;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author,version,about,long_about = None)]
struct Args {
    #[clap(short,long,value_parser)]
    config: Option<PathBuf>,
    
    #[clap(short,long,value_parser)]
    onlyPwd: bool,
}

fn main() {
    // Load args
    let args = Args::parse();

    // Validate config path
    if let Some(config_path) = args.config.as_deref() {
        if args.onlyPwd == true {
            println!("onlyPwd flag enabled~\n\n");
        }
    } else {
        exit(1);
    }
}
