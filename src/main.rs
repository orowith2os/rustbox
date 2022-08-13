//mod create;
//mod enter;
//mod list;
//mod stop;
//mod rm;
//mod ephemeral;
const VERSION: i32 = 0; //change this on version updates

fn main() {
    let availablecommands = "create
        enter
        list
        stop
        rm
        version";
    println!("Error: invalid command");
    println!("Rustbox version: {VERSION}");
    println!("Choose one of the available commands: \n 
        {availablecommands}");
 
}
