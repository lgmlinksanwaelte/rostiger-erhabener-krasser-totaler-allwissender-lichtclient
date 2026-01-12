mod gui;

use std::process::Command;
fn main() {
    gui::run();

    let status = Command::new("echo")
        .arg("test")
        .status()
        .expect("test");

    println!("Exit-Code: {}", status);
}
