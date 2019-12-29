// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("echo me")));
    println!("{:?}", Message::Move { x: 14, y: 15 });
    println!("{:?}", Message::ChangeColor(25, 25, 25));
}
