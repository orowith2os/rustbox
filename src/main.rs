

const VERSION: &str = env!("CARGO_PKG_VERSION");

mod args;

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
