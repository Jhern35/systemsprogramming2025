use serde::Deserialize;

trait Pricing {
    fn fetch_price(&self) -> f64;
    fn save_to_file(&self);
}

struct Bitcoin {
    id:"Bitcoin".to_string(),
}

impl Pricing for Bitcoin {  
    fn fetch_price(&self, url: String) -> f64 {
        //Get the price and return it (error handling is important here)
    }

    fn save_to_file(&self, url: String, price: f64) {
        //Save price to file
        //Possible format to save:
        //"self.id current price: price"
    }
}

struct Ethereum {
    id:"Ethereum".to_string(),
}

impl Pricing for Ethereum {
    fn fetch_price(&self, url: String) -> f64 {
        //Get the price and return it (error handling is important here)
    }

    fn save_to_file(&self, url: String, price: f64) {
        //Save price to file
        //Possible format to save:
        //"self.id current price: price"
    }
}

struct SP500 {
    id:"S&P 500".to_string(),
}

impl Pricing for SP500 {
    fn fetch_price(&self, url: String) -> f64 {
        //Get the price and return it (error handling is important here)
    }

    fn save_to_file(&self, url: String, price: f64) {
         //Save price to file
        //Possible format to save:
        //"self.id current price: price"
    }
}

fn main() {
    println!("Hello, world!");
}
