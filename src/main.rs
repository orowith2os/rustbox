pub(crate) use clap::Parser;
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[clap(version, long_about = None)]
struct Args {
    /// Print Rustbox's version
    #[clap(short = 'v', long = "version", value_parser)]
    version: bool,
}
fn main() {
    let args = Args::parse();
    if args.version {}
    {
        println!("Rustbox version: {VERSION}");
    }
}


// fn noarguments() {
//     let availablecommands = "create
//         enter
//         list
//         stop
//         rm
//         version";
//     println!("Error: invalid command");
//     println!("Rustbox version: {VERSION}");
//     println!(
//         "Choose one of the available commands: \n 
//         {availablecommands}"
//     );
// } I've commented these lines out, as Clap's built-in --help is good enough.
