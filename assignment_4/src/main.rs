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

struct Bitcoin {
    file_path: String,
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

impl Pricing for Bitcoin {  
    fn fetch_price(&self) -> f64 {
        let url = "https://query2.finance.yahoo.com/v8/finance/chart/BTC-USD";

        let resp = match ureq::get(url).call() {
            Ok(response) => response,
            Err(e) => { 
                println!("Error with retrieving price: {}", e);
                return -1.0;
            }
        };

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
        
        let mut file = OpenOptions::new().append(true).open(&self.file_path).expect("Error opening file!");
        if price == -1.0 {
            writeln!(file, "Unable to retrieve price of Bitcoin").ok();
        }
        else {
            println!("Bitcoin current price: {}", price);
            writeln!(file, "Bitcoin current price: {}", price).ok();
        }
    }
}


struct Ethereum {
    file_path: String,
}

impl Ethereum {
    fn new() -> Self {
       let file_path = "Ethereum_price.txt".to_string();
       OpenOptions::new().create(true).append(true).open("Ethereum_price.txt".to_string()).expect("Unable to create file");

       Self {
        file_path,
       }

    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> f64 {
            let url = "https://query2.finance.yahoo.com/v8/finance/chart/ETH-USD";

            let resp = match ureq::get(url).call() {
                Ok(response) => response,
                Err(e) => { 
                    println!("Error with retrieving price: {}", e);
                    return -1.0;
                }
            };

            let price: Chart = match resp.into_json() {
                Ok(eth) => eth,
                Err(e) => {
                    println!("Error with retrieving price: {}", e);
                    return -1.0;
                }
            };
            
            if let Some(eth) = price.chart.result.first() {
                eth.meta.regularMarketPrice
            }
            else {
                -1.0
            }

    }

    fn save_to_file(&self, price: f64) {
        
        let mut file = OpenOptions::new().append(true).open(&self.file_path).expect("Error opening file!");
        if price == -1.0 {
            writeln!(file, "Unable to retrieve price of Ethereum").ok();
        }
        else {
            println!("Ethereum current price: {}", price);
            writeln!(file, "Ethereum current price: {}", price).ok();
        }
    }
}

struct SP500 {
    file_path: String,
}

impl SP500 {
    fn new() -> Self {
       let file_path = "SP500_price.txt".to_string();
       OpenOptions::new().create(true).append(true).open("SP500_price.txt".to_string()).expect("Unable to create file");

       Self {
        file_path,
       }

    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> f64 {
            let url = "https://query2.finance.yahoo.com/v8/finance/chart/%5EGSPC";

            let resp = match ureq::get(url).call() {
                Ok(response) => response,
                Err(e) => { 
                    println!("Error with retrieving price: {}", e);
                    return -1.0;
                }
            };

            let price: Chart = match resp.into_json() {
                Ok(sp) => sp,
                Err(e) => {
                    println!("Error with retrieving price: {}", e);
                    return -1.0;
                }
            };
            
            if let Some(sp) = price.chart.result.first() {
                sp.meta.regularMarketPrice
            }
            else {
                -1.0
            }

    }

    fn save_to_file(&self, price: f64) {
        
        let mut file = OpenOptions::new().append(true).open(&self.file_path).expect("Error opening file!");
        if price == -1.0 {
            writeln!(file, "Unable to retrieve price of S&P 500").ok();
        }
        else {
            println!("S&P 500 current price: {}", price);
            writeln!(file, "S&P 500 current price: {}", price).ok();
        }
    }
}

fn main() {
    let btc = Bitcoin::new();
    let eth = Ethereum::new();
    let sp = SP500::new();

    let price_checker:Vec<&dyn Pricing> = vec![&btc, &eth, &sp];

    loop {

        for currency in &price_checker {
            let price = currency.fetch_price();
            currency.save_to_file(price);
        }

        thread::sleep(Duration::from_secs(10));
        
    }
}
