//use std::process::Command;
use which::which;

fn host-exec() {
    let hostspawn = which("host-spawn");
    println!("{}", hostspawn)
}