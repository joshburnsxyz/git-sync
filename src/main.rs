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
    let pwd = env::current_dir().unwrap();

    // Validate config path
    if let Some(config_path) = args.config.as_deref() {    
        if args.recurse == true {
            let paths: Vec = git::find_repos();
            for e in paths {
                // set pwd to the selected path.
                env::set_current_dir(&e.path()); // TODO: Error check this function call
                
                if git::is_repo(&e) {
                    git::pull();
                    git::push();
                } else {
                    println!("No git repo found here...");
                }
                println!("{:#?}", e);

                // set pwd to the selected path.
                env::set_current_dir(&pwd);
            }
        }
    } else {
        exit(1);
    }
}
