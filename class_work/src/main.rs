fn fahernheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0/9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0/5.0) + 32.0
}

fn main() {
    let mut x:f64 = 32.0;
    let mut counter = 0;

    x = fahernheit_to_celsius(x);
    println!("{}", x);

    loop {
      x = celsius_to_fahrenheit(x);
      x += 1.0;
      counter += 1;

      x = fahernheit_to_celsius(x);
      println!("{}", x);

      if counter >= 5 { break; }
    }
}
