extern crate clap;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short,long,value_parser)]
    name: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args);
}