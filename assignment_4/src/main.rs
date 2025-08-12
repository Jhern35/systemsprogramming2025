use std::fs::OpenOptions;
use std::io::Write;
use std::{thread, time::Duration};
use serde::Deserialize;

trait Pricing {
    fn fetch_price(&self) -> f64;
    fn save_to_file(&self, price:f64);
}

#[derive(Debug, Deserialize)]
struct Chart {
    chart: Result,
}

#[derive(Debug, Deserialize)]
struct Result {
    result: Vec<Data>,
}

#[derive(Debug, Deserialize)]
struct Data {
    meta: Price,
}

#[derive(Debug, Deserialize)]
struct Price {
    regularMarketPrice: f64,
}

#[derive(Debug)]
struct Bitcoin {
    file_path: String,
}

impl Pricing for Bitcoin {  
    fn fetch_price(&self) -> f64 {
        //Makes a HTTP request to the given webiste
        let url = "https://query2.finance.yahoo.com/v8/finance/chart/BTC-USD";

        let resp = match ureq::get(url).call() {
            Ok(response) => response,
            Err(e) => { 
                println!("Error with retrieving price: {}", e);
                return -1.0;
            }
        };

        //It reads the file and returns a variable and returns it.
        let price: Chart = match resp.into_json() {
            Ok(btc) => btc,
            Err(e) => {
                println!("Error with retrieving price: {}", e);
                return -1.0;
            }
        };
        
        if let Some(btc) = price.chart.result.first() {
            btc.meta.regularMarketPrice
        }
        else {
            -1.0
        }

    }

    fn save_to_file(&self, price: f64) {
        //Creating a file to store all incoming prices and panicking if file can't be created or it fails to write to it
        //Save price to file while handling for errors
        let mut file = OpenOptions::new().append(true).open(&self.file_path).expect("Error opening file!");
        if price == -1.0 {
            writeln!(file, "Unable to retrieve price of Bitcoin").ok();
        }
        else {
            writeln!(file, "Bitcoin current price: {}", price).ok();
        }
    }
}

impl Bitcoin {
    fn new() -> Self {
       let file_path = "Bitcoin_price.txt".to_string();
       OpenOptions::new().create(true).append(true).open("Bitcoin_price.txt".to_string()).expect("Unable to create file");

       Self {
        file_path,
       }

    }
}

// struct Ethereum {
//     price:f64,
// }

// impl Pricing for Ethereum {
//     fn fetch_price(&self, url: String) -> f64 {
//         //Get the price and return it (error handling is important here)
//     }

//     fn save_to_file(&self, url: String, price: f64) {
//         //Save price to file
//         //Possible format to save:
//         //"self.id current price: price"
//     }
// }

// struct SP500 {
//     price:f64,
// }

// impl Pricing for SP500 {
//     fn fetch_price(&self, url: String) -> f64 {
//         //Get the price and return it (error handling is important here)
//     }

//     fn save_to_file(&self, url: String, price: f64) {
//          //Save price to file
//         //Possible format to save:
//         //"self.id current price: price"
//     }
// }

fn main() {
    let btc = Bitcoin::new();
    let price_checker:Vec<&dyn Pricing> = vec![&btc];

    loop {

        for currency in &price_checker {
            let price = currency.fetch_price();
            currency.save_to_file(price);
        }

        thread::sleep(Duration::from_secs(10));
        
    }
}
