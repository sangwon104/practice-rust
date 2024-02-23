fn main() {
    // enum_basic();
    // enum_option();
    enum_match();
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn enum_match() {
    println!("penny coin : {}", value_in_cents(Coin::Penny));
    println!("quarter coin : {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn enum_option() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("some number : {:?}", some_number);
    println!("some char : {:?}", some_char);
    println!("absent number : {:?}", absent_number);
}

enum IpAddr {
    V4(String),
    V6(String)
}

fn enum_basic() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("call : {:?}", &self);        
    }
}
