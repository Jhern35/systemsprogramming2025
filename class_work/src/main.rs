struct Car {
    name:String, 
    model:String,
    year:u32,
}

impl Car {
    fn new(n:String, m:String, y:u32) -> Car {
        Car {
            name:n,
            model:m,
            year:y,
        }
    }

    fn show_info(&self) {
        println!("My car maker and models is {}{} and year is {}", self.name, self.model, self.year);
    }

    fn change_year(&mut self, new_year:u32){
        self.year = new_year;
    }
}

fn main() {
    let mut my_car = Car::new ("Ford".to_string(), "350".to_string(),  2025);
    my_car.show_info();
    my_car.change_year(2023);
    my_car.show_info();
}