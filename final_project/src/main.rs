use std::{thread, time::{Duration, Instant}};
use chrono::{DateTime, Utc};
use std::sync::{mpsc, Arc, Mutex};
use ureq::Agent;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

#[derive(Debug)]
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: DateTime<Utc>,
}

fn check_status(url: String, timeout: Duration, tries:u16) -> WebsiteStatus {
    let start = Instant::now();
    let mut resp = None;
    let agent: Agent = Agent::new();             

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
    
    let end = start.elapsed();
    let status: Result<u16, String> = resp.unwrap();

    WebsiteStatus {
        url,
        status,
        response_time: end,
        timestamp: Utc::now(),
    }
}

fn inputs() -> Duration {
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
    let buffer = BufReader::new(file);

    let mut urls: Vec<String> = Vec::new();
    
    for lines in buffer.lines() {
        match lines {
            Ok(url) => urls.push(url),
            Err(e) => println!("Unable to get url: {}", e),
        };
    }

    let timeout = inputs();
    let num_workers = 10;
    let tries: u16 = 3;

    let (tx, rx) = mpsc::channel::<String>();
    let (result_tx, result_rx) = mpsc::channel::<WebsiteStatus>();

    let rx = Arc::new(Mutex::new(rx));

    for _ in 0..num_workers {
        let rx = Arc::clone(&rx);
        let result_tx = result_tx.clone();
        let timeout = timeout.clone();
        
        thread::spawn(move || {
            loop {
                let url = match rx.lock().unwrap().recv() {
                    Ok(url) => url,
                    Err(_) => break, 
                };

                let status = check_status(url, timeout, tries);
                result_tx.send(status).unwrap();
            }
        });
    }

    drop(result_tx); 

    for url in urls {
        tx.send(url).unwrap();
    }
    drop(tx); 

    for res in result_rx {
        println!("{:?}", res);
    }
    
}
