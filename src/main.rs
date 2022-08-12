//mod create;
//mod enter;
//mod list;
//mod stop;
//mod rm;
//mod ephemeral;

fn main() {
    let version = 0;
    let availablecommands = "create
        enter
        list
        stop
        rm
        version";
    println!("Error: invalid command");
    println!("Rustbox version: {version}");
    println!("Choose one of the available commands: \n 
        {availablecommands}");
 
}
