fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constant
    let mut x = 5; // mutable variable
    println!("the value of x is : {x}");
    x = 6;
    println!("the value of x is : {x}");
    println!("3 hours is {THREE_HOURS_IN_SECONDS} seconds");

    // shadowing
    let x = "hello shadowing";
    println!("{x}");
    {
        let x = "hi shadowing";
        println!("{x}");
    }
    println!("{x}");

    // possible
    let spaces = "     "; // String
    let spaces = spaces.len(); // Number, shadowing에 의해 변수가 가려지므로 가능
    println!("{spaces}");
    // impossible
    // let mut spaces = "     "; // String
    // spaces = spaces.len(); // 이미 String type이라 Number type을 담을 수 없음.
    
}
