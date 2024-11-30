#[allow(dead_code)]
#[warn(unused_variables)]
enum Message {
    Fine,
    Okaish(),
    Great(String),
}
#[derive(Debug)]
enum Ecells{
    MyInt(i32),
    MyText(String),
}


fn main() {
    let xs_mssg: Message = Message::Great("Hello".to_string());
    let _fine_msg = Message::Fine;
    let _ok_msg = Message::Okaish();
    match xs_mssg {
        Message::Fine => print!("Fine likha hai"),
        Message::Okaish() => print!("Okaish (without anything inside it.)"),
        Message::Great(some_str) => print!("Great (with string as {some_str})"),
    }



    let row:Vec<Ecells> = vec![Ecells::MyInt(8), Ecells::MyText(String::from("Yo!!!"))];
    
    match row.get(1){
        Some(Ecells::MyInt(value)) => println!("gotint...!!{value}"),
        Some(Ecells::MyText(text)) => println!("gotstring...!!{text}"), // Extract the string value directly
        None => println!("gotNONE...!!")
    }
    
}
