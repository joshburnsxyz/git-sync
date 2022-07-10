extern crate clap;

use std::env;
use std::path::PathBuf;
use std::process::exit;
use clap::Parser;
use git_sync::git;

#[derive(Parser, Debug)]
#[clap(author,version,about,long_about = None)]
struct Args {
    #[clap(short,long,value_parser)]
    config: Option<PathBuf>,
    
    #[clap(short,long)]
    recurse: bool,
}

fn main() {
    // Load args
    let args = Args::parse();

    // Validate config path
    if let Some(config_path) = args.config.as_deref() {    
        if args.recurse == true {
            let paths: Vec = git::find_repos();
            let pwd = env::current_dir().unwrap();
            for e in paths {
                println!("{:#?}", e);
            }
        }
    } else {
        exit(1);
    }
}
