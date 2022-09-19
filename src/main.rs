pub(crate) use clap::Parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");

mod host-exec/host-exec;

#[derive(Parser, Debug)]
#[clap(version, long_about = None)]
struct Args {
    /// Print Rustbox's version.
    #[clap(short = 'v', long = "version")]
    version: bool,

    /// Enter a container.
    #[clap(long = "enter")]
    enter: bool,

    /// List all containers.
    #[clap(short = 'l', long = "list")]
    list: bool,

    /// Stop a container, or all containers.
    #[clap(long = "stop")]
    stop: bool,

    /// Remove a container.
    #[clap(long = "remove")]
    remove: bool,

    /// Make a temporary container.
    #[clap(long = "ephemeral")]
    ephemeral: bool,
}

fn main() {
    let args = Args::parse();

    if args.version {}
    {
        println!("Rustbox version: {VERSION}");
    }
    if args.enter {}
    {
        println!("Enter a container. Not yet implemented.")
    }
    if args.list {}
    {
        println!("List all containers. Not yet implemented.")
    }
    if args.stop {}
    {
        println!("Stop a container, or all containers. Not yet implemented.")
    }
    if args.remove {}
    {
        println!("Remove a container. Not yet implemented.")
    }
    if args.ephemeral {}
    {
        println!(
            "Make a temporary container that lasts for until you exit it. Not yet implemented."
        )
    }
}

// fn noarguments() {
//     let availablecommands = "create
//         rm
//     println!("Error: invalid command");
//     println!("Rustbox version: {VERSION}");
//     println!(
//         "Choose one of the available commands: \n
//         {availablecommands}"
//     );
// } I've commented these lines out, as Clap's built-in --help is good enough.
