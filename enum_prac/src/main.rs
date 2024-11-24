enum Message {
    Fine,
    Okaish(),
    Great(String),
}

fn main() {
    let xs_mssg: Message = Message::Great("Hello".to_string());
    let fine_msg = Message::Fine;
    let ok_msg = Message::Okaish();
    match xs_mssg {
        Message::Fine => print!("Fine likha hai"),
        Message::Okaish() => print!("Okaish (without anything inside it.)"),
        Message::Great(some_str) => print!("Great (with string as {some_str})"),
    }
}
