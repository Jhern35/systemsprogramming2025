fn module_1() {
    fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
        let mut i = low;

        while i <= high {
            *total += i;
            i += step;
        }
    }

    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);

}

fn module_2() {
    fn most_frequent_word(text: &str) -> (String, usize) {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut max_count:usize = 0;
        let mut max_word = "";

        for word in &words {
            let mut curr:usize = 0;
            for i in 0..words.len() {
                if words[i] == *word { curr += 1; }
            }
            if curr > max_count { 
                max_count = curr; 
                max_word = *word;
            }
        }

        (max_word.to_string(), max_count)
    }

    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}


fn main() {
    module_1();
    module_2();
}
