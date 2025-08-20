use std::{thread, time::Duration};
use serde::Deserialize;
use chrono::Utc;
use std::sync::mpsc;
use ureq;

struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: DateTime<Utc>,
}

fn check_status(url: String, timeout: Duration) -> WebsiteStatus {
    let start = Instant::now();
    let resp = match ureq::get(&url)
        .timeout_connect(timeout)
        .timeout_read(timeout)
        .call() {
            if Ok(status) => { status.status(); break;},
            if Err(e) => { if retries == 0 { e }; else retries -= 1; },
        };
    
    let end = start.elasped();

    WebsiteStatus {
        url,
        status,
        response_time: end,
        timestamp: Utc::now(),
    }
}


fn main() {
    let urls = 

    let timeout, num_workers = time_input();

    let (link, output) = mspc::channel::<String>();
    let (input, result) = mspc::channel::<WebsiteStatus>();

    for _ in 0..num_workers {
        let output = output.clone();
        let input = input.clone();
        
        thread::spawn(move || {
            for url in output.iter() {
                let status = check_website(url, timeout);
                input.send(status).unwrap();
            }
        });
    }

    for url in urls {
        link.send(url).unrwap();
    }
    
    drop(link);
    drop(input);

    for res in result {
        println!("{:?}", res);
    }
}
