use clap::Parser;
//use std::env::args;

const VERSION: i32 = 0; //change this on version updates

#[derive(Parser, Debug)]
#[clap(version, long_about = None)]
struct Args {
    /// Print Rustbox's version
    #[clap(long = "version", value_parser)]
    version: bool,
}

fn main() {
    let args = Args::parse();

    if args.version == true {
        println!("Rustbox version: {VERSION}");
    }
}

fn noarguments() {
    let availablecommands = 
       "create
        enter
        list
        stop
        rm
        version";
    println!("Error: invalid command");
    println!("Rustbox version: {VERSION}");
    println!(
        "Choose one of the available commands: \n 
        {availablecommands}"
    );
}
