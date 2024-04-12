fn main() {
    // enum_basic();
    // enum_option();
    // enum_match();
    // catch_all_pattern();
    if_let();
}

fn if_let() {
    let config_max = Some(3u8);

    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => ()
    // }
    if let Some(max) = config_max {
         Some(max) => println!("The maximum is configured to be {}", max),
    }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
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
    // println!("quarter coin : {}", value_in_cents(Coin::Quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn catch_all_pattern() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // _ => reroll()
        _ => () // 어디에도 포함되지 않는 경우, 동작하지 않음을 명시
    }
}

fn add_fancy_hat() {
    println!("add fancy hat");
}

fn remove_fancy_hat() {
    println!("remove fancy hat");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
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
