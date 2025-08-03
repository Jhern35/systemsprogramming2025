use std::process::{Command, Stdio};

enum ProcessOperation {
    Start(String),
    Pipe(String, String),
}

fn perform_process_operation(operation: ProcessOperation) {
    match operation {
        ProcessOperation::Start(cmd) => {
            let output = Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .output()
                .unwrap();

            println!("Process output: {}", String::from_utf8_lossy(&output.stdout));
        },
        ProcessOperation::Pipe(cmd1, cmd2) => {
            let process1 = Command::new("sh")
                .arg("-c")
                .arg(&cmd1)
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let output = Command::new("sh")
                .arg("-c")
                .arg(&cmd2)
                .stdin(Stdio::from(process1.stdout.unwrap()))
                .output()
                .unwrap();

            println!("Piped output: {}", String::from_utf8_lossy(&output.stdout));
        }
    }
}

fn main() {
    let operations = vec![
        ProcessOperation::Start(String::from("ls -l")),
        ProcessOperation::Pipe(String::from("echo 'Hello, World!'"), String::from("wc -w")),
    ];

    for op in operations {
        perform_process_operation(op);
    }
}