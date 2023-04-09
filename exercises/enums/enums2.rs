// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.


#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo(String),
    Move { x: u8, y: u8 },
    ChangeColor(u8, u8, u8),
    Unit {},
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
        Message::Unit {},
    ];

    for message in &messages {
        message.call();
    }
}
