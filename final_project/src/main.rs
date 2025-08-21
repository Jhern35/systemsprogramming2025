use std::io::{BufReader, BufRead, Write};
use std::io;
use std::{thread, time::{Duration, Instant}};
use std::sync::{mpsc, Arc, Mutex};
use ureq::Agent;
use chrono::{DateTime, Utc};
use std::fs::File;


#[derive(Debug)]
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: DateTime<Utc>,
}

fn check_website(url: String, timeout: Duration, tries:u16) -> WebsiteStatus {
    let start = Instant::now();
    let mut resp = None;
    let agent: Agent = Agent::new();   
    let timestamp = Utc::now();          

    for attempt in 1..=tries {
        match agent.get(&url)
            .timeout(timeout)
            .call() {
                Ok(status) => { resp = Some(Ok(status.status())); break; },
                Err(e) => { 
                    println!("Unable to get website status retrying.."); 
                    if attempt == tries {
                        println!("Ran out of tries for this url: {}", url);
                        resp = Some(Err(e.to_string()));
                    }
                }
            };
    }
    
    let response_time = start.elapsed();
    let status: Result<u16, String> = resp.unwrap();

    WebsiteStatus {
        url,
        status,
        response_time,
        timestamp,
    }
}

fn inputs() -> Duration {
    print!("Enter max amount of time for program to call the website: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();

    if let Err(e) = io::stdin().read_line(&mut input) {
        println!("Input error {} resorting to default value", e);
        return Duration::from_secs(5);
    };

    match input.trim().parse::<u16>() {
        Ok(num) => Duration::from_secs(num.into()),
        Err(e) => { println!("Input error {} resorting to default value", e);
                Duration::from_secs(5)
            },
    }
}


fn main() {
    let file = File::open("Urls.txt").expect("Couldn't open file");
    let reader = BufReader::new(file);

    let mut urls: Vec<String> = Vec::new();
    
    for lines in reader.lines() {
        match lines {
            Ok(url) => urls.push(url),
            Err(e) => println!("Unable to get url: {}", e),
        };
    }

    let timeout = inputs();
    let workers = 50;
    let tries: u16 = 3;

    let (url_tx, url_rx) = mpsc::channel::<String>();
    let (output_tx, output_rx) = mpsc::channel::<WebsiteStatus>();

    let url_rx = Arc::new(Mutex::new(url_rx));

    for _ in 0..workers {
        let url_rx = Arc::clone(&url_rx);
        let output_tx = output_tx.clone();
        
        thread::spawn(move || {
            loop {
                let url = match url_rx.lock().unwrap().recv() {
                    Ok(url) => url,
                    Err(_) => break, 
                };

                let status = check_website(url, timeout, tries);
                output_tx.send(status).unwrap();
            }
        });
    }

    drop(output_tx); 

    for url in urls {
        url_tx.send(url).unwrap();
    }

    drop(url_tx); 

    for res in output_rx {
        match res.status {
            Ok(status) => {
                println!("✅ In {:?} url: {} responded with HTTP:{} @[{}]", res.response_time, res.url, status, res.timestamp);
            }
            Err(e) => {
                println!("❌ In {:?} url: {} failed with {} @[{}]", res.response_time, res.url, e, res.timestamp);
            }
        }
    }
    
}
