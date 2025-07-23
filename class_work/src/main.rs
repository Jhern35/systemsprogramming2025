fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
    let mut res = "".to_string();
    res.push_str(s1);
    res.push_str(s2);
    res
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}