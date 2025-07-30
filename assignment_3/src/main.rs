use std::process::Command;
use std::io;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;
use std::io::{BufReader, BufRead};
use chrono::{Local};

struct LinuxAgent {
    path: String,
}

impl LinuxAgent {
    fn new() -> LinuxAgent {
        OpenOptions::new().read(true)
            .create(true).append(true).open("History.txt")
            .expect("Error with file!");

        LinuxAgent {
            path: "History.txt".to_string(),
        }
    }

    fn executing_os_commands_linux(&self, command_full: &str) {
        let command: Vec<&str> = command_full.split_whitespace().collect();
        if command.len() < 1 { println!("No command entered"); return; }
        let output = { 
            Command::new(&command[0])
                .args(&command[1..command.len()])
                .output()
            };
        match output {
            Ok(result) => {
                let stdout = match String::from_utf8(result.stdout) {
                    Ok(s) => s,
                    Err(e) => { println!("Error {}", e); return; }
                };
                
                let stderr = match String::from_utf8(result.stderr) {
                    Ok(s) => s,
                    Err(e) => { println!("Error {}", e); return; }
                };

                self.save_results(format!("{} {} {}", command_full, stdout, stderr));
            }
            Err(e) => {
                println!("Error {}!", e);
            }
        }
    }

    fn accept_linux_command_from_user() -> String {
        print!("user-1: ~$ ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading line");
        input.trim().to_string()
    }

    fn save_results(&self, content: String) {
        let mut file = OpenOptions::new().append(true).open(&self.path).expect("Unable to open file!");
        let time = Local::now().to_string();
        let full_history = format!("{} @{}", content, time);
        file.write_all(full_history.as_bytes());
    }

    fn show_results(&self) {
        let file = File::open(&self.path).expect("Unable to open file!");
        let buffer = BufReader::new(file);

        for line in buffer.lines() {
            match line{
                Ok(s) => println!("{}", s),
                Err(e) => println!("Error with printing line {}", e),
            };
        }
    }
}

fn main() {
    // TODO: Implement the main program loop
    // 1. Create a LinuxAgent instance
    // 2. Enter a loop to accept and execute commands
    // 3. Break the loop when the user wants to stop
    // 4. Show the results
    let user = LinuxAgent::new();
    let inp = LinuxAgent::accept_linux_command_from_user();
    user.executing_os_commands_linux(&inp);
    user.show_results();
}