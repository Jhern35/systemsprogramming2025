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
        let time = Local::now().to_string();
        match Command::new(&command[0]).args(&command[1..command.len()]).output() {
            Ok(output) => {
                let std = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                let full_history = format!("{}: {} - {} {}\n", time, command_full, std, 
                stderr);
                self.save_results(full_history);
            }
            Err(e) => {
                println!("Inavlid command: {}", e);
            }
        }

    }

    fn accept_linux_command_from_user() -> String {
        print!("user-1: ~$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading line");
        input.trim().to_string()
    }

    fn save_results(&self, content: String) {
        let mut file = OpenOptions::new().append(true).open(&self.path).expect("Unable to open file!");
        _= file.write_all(content.as_bytes());
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
    let user = LinuxAgent::new();
    loop {
        let inp = LinuxAgent::accept_linux_command_from_user();
        io::stdout().flush().unwrap();
        if inp == "exit".to_string() { break; }
        user.executing_os_commands_linux(&inp);
    }
    user.show_results();
}