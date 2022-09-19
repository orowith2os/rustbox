//use std::process::Command;
use which::which;

fn hostexec() {
    let hostspawn = which("host-spawn");
    println!("{:?}", hostspawn)
}
