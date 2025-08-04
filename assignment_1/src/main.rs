fn module_1() {
    fn fahrenheit_to_celsius(f: f64) -> f64 {
        (f - 32.0) * 5.0/9.0
    }

    fn celsius_to_fahrenheit(c: f64)  -> f64 {
        (c * 9.0/5.0) + 32.0
    }

    let mut x:f64;
    let mut count:u32 = 0;

    x = fahrenheit_to_celsius(32.0);
    println!("{}", x);

    loop {
        x = celsius_to_fahrenheit(x);

        x += 1.0;
        x = fahrenheit_to_celsius(x);
        println!("{}", x);
        count += 1;

        if count == 5 { break; }
    }
}

fn module_2() {
    fn is_even(n: i32) -> bool {
        if n % 2 == 0 { 
            return true;
        }
        else {
            return false;
        }
    }

    let arr = [1, 2, 3, 5, 6, 8, 10, 12, 15, 20];
    let mut count = 0;
    let mut sum:i32 = 0;
    let mut biggest:i32 = 0;

    for i in 0..arr.len() {
        let mut s:String = "".to_string();
        if arr[i] % 3 == 0 { s.push_str("Fizz"); }
        if arr[i] % 5 == 0 { s.push_str("Buzz"); }
        
        if s.chars().count() == 0 && is_even(arr[i]) {
            println!("{} is even", arr[i]);
        }
        else if s.chars().count() == 0 && !is_even(arr[i]){
            println!("{} is odd", arr[i]);
        }
        else {
            println!("{}", s);
        }
    }

    while count < 10 {
        sum += arr[count];
        count += 1;
    }

    count = 0;

    loop {
        if arr[count] > biggest { biggest = arr[count]; }
        count += 1;
        if count == 10 { break; }
    }

    println!{"the sum is {} and the biggest value is {}", sum, biggest};

}

fn module_3() {
    fn check_guess(guess: i32, secret: i32) -> i32 {
        if guess == secret { return 0; }
        else if guess > secret { return 1; }
        else { return -1; }
    }

    let secret:i32 = 110;
    let mut guess:i32 = 100;
    let mut counter:i32 = 0;

    loop {
        if check_guess(guess, secret) == 0 { break; }
        counter += 1;
        guess += 1;
    }
    println!("It took you {} tries", counter);
}

fn main() {
    module_1();
    module_2();
    module_3();
}
