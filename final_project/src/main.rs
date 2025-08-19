use std::{thread, time::Duration};
use serde::Deserialize;
use chrono::Utc;
use std::sync::mspc;
use ureq;

struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: DateTime<Utc>,
}

fn check_status(url: String, timeout: Duration) -> WebsiteStatus {
    let start = Instant::now();
    let resp = ureq::get(&url)
                .timeout_connect(timeout)
                .timeout_read(timeout)
                .call();
    
    let end = start.elasped();

    let status = match resp {
        Ok(result) => Ok(resp.status()),
        Err(ureq::Error::StatusCode(code, _response)) => Err(code),
        Err(e) => Err(format!("Network error {:?}", e)),
    };

    WebsiteStatus {
        url,
        status,
        response_time: end,
        timestamp: Utc::now(),
    }
}


fn main() {
    println!("Hello, world!");
}
