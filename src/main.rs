use std::process::Command;

fn record() {
    let output = Command::new("cmd")
        .arg("/C")
        .arg("echo")
        .arg("Hello world")
        .output()
        .expect("Failed to execute command");

    let string = output.stdout;

    println!("{}", String::from_utf8(string).unwrap());
}

fn main() {
    record();
}
