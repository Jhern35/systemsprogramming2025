#[allow(dead_code)]
fn generics_in_struct() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5u8, y: 10u8 };
    let float = Point { x: 1.0f32, y: 4.0 };
    let words = Point { x: 'c', y: 'z' };

    println!("int Point: {:?} float Point: {:?} word Point: {:?}", integer, float, words);

    #[derive(Debug)]
    struct User<T, U> {
        name: T,
        y: U,
    }

    let user1 = User { name: "Vandam", y: 35 };
    let user2 = User { name: "James Bond".to_string(), y: 'b' };

    println!("User1: {:?} User2: {:?}", user1, user2);
}

fn main() {
   generics_in_struct();
}