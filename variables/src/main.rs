fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constant
    let mut x = 5; // immutable variable
    println!("the value of x is : {x}");
    x = 6;
    println!("the value of x is : {x}");
    println!("3 hours is {THREE_HOURS_IN_SECONDS} seconds");
}
