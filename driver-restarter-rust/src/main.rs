use std::env;
use std::io::{self};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("\nNo device instance path was given.");
        println!("Please read the manual for more information");
        let mut manual = Command::new("cmd");
        manual.args([
            "/C",
            "start",
            "https://github.com/lamborghinigamer1/Driver-Restarter/blob/main/README.md",
        ]);

        match manual.output() {
            Ok(o) => unsafe {
                println!("{}", String::from_utf8_unchecked(o.stdout));
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        pause();
    } else {
        let device_identifier = &args[1];

        let mut disable = Command::new("pnputil");
        disable.arg("/disable-device").arg(device_identifier);
        let mut enable = Command::new("pnputil");
        enable.arg("/enable-device").arg(device_identifier);

        match disable.output() {
            Ok(o) => unsafe {
                println!("{}", String::from_utf8_unchecked(o.stdout));
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        match enable.output() {
            Ok(o) => unsafe {
                println!("{}", String::from_utf8_unchecked(o.stdout));
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
fn pause() {
    println!("Press enter to exit...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}
