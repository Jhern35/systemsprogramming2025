use std::fs::File;
use std::io::Write;
use std::io::prelude::*;
use std::io::{self, Read, Write};

struct Stduent {
    name:String,
    major:String,
}

impl Student {
    fn from_file(path: &str) -> Student {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let major = lines.next().unwrap().to)string();

        Student {name, major}
    }
}

fn read_from_console() {
    print!("What is your name?");
    
}

fn reading_from_file() {
    let data = Student::from_file("person.txt") 
}

fn main() {
   reading_from_console();
    reading_from_file();
}




// struct Car {
//     name:String, 
//     model:String,
//     year:u32,
// }

// impl Car {
//     fn new(n:String, m:String, y:u32) -> Car {
//         Car {
//             name:n,
//             model:m,
//             year:y,
//         }
//     }

//     fn show_info(&self) {
//         println!("My car maker and models is {}{} and year is {}", self.name, self.model, self.year);
//     }

//     fn change_year(&mut self, new_year:u32){
//         self.year = new_year;
//     }
// }

// fn main() {
//     let mut my_car = Car::new ("Ford".to_string(), "350".to_string(),  2025);
//     my_car.show_info();
//     my_car.change_year(2023);
//     my_car.show_info();
// }