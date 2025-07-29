use std::process::Command;
use std::io;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;
use std::io::{BufReader, BufRead};

struct LinuxAgent {
    path: String,
}

impl LinuxAgent {
    fn new() -> LinuxAgent {
        LinuxAgent {
            path: "/opt/Jhern/LinuxAgent".to_string(),
        }

        let mut file = File:create("History.txt");
    }

    fn executing_os_commands_linux(&self, command_full: &str) {
       let output = { 
        Command::new(self.path)
            .args(command_full.parse())
            .output()
        };

        save_results(output.to_string() + output.stdout.to_string());
    }

    fn accept_linux_command_from_user() -> String {
        // TODO: Implement user input
        // Prompt the user for a Linux command and return it
    }

    fn save_results(&self, content: String) {
        // TODO: Implement saving results
        // Append the command and its output to the file
    }

    fn show_results(&self) {
        // TODO: Implement showing results
        // Read and display the contents of the command history file
    }
}

fn main() {
    // TODO: Implement the main program loop
    // 1. Create a LinuxAgent instance
    // 2. Enter a loop to accept and execute commands
    // 3. Break the loop when the user wants to stop
    // 4. Show the results
}