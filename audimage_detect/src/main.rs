use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

fn main() {
    let mut child_shell = Command::new("/home/jovie/projects/rust/audimage/target/debug")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let child_out = BufReader::new(child_shell.stdout.as_mut().unwrap());

    for line in child_out.lines() {
        match line {
            Ok(output) => println!("{}", output),
            Err(e) => eprintln!("Error reading line... {}", e),
        }
    }
}
