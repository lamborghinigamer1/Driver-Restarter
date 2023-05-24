use std::env;
use std::process::Command;
use std::io::{self};

fn pause() {
    println!("Press any key to exit...");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}
fn main() {

    let allargs: Vec<String> = env::args().collect();

    if allargs.len() <= 1 {

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
        for args in allargs.iter().skip(1) {
            let device_identifier = &args;

            let mut disable = Command::new("pnputil");

            disable.arg("/disable-device");
            disable.arg(device_identifier);

            let mut enable = Command::new("pnputil");

            enable.arg("/enable-device");
            enable.arg(device_identifier);

            match disable.output() {
                Ok(output) => unsafe {
                    println!("{}", String::from_utf8_unchecked(output.stdout));
                },

                Err(er) => {
                    println!("Error: {}", er);
                }
            }

            match enable.output() {
                Ok(output) => unsafe {
                    println!("{}", String::from_utf8_unchecked(output.stdout));
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}
