// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
enum Message {
    Quit,
    // An enum variant is a struct, which specifies the type
    Move {x: i32, y: i32},
    // specify the type of enum variant
    Echo (String),
    // unit like struct, specifies the type
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
