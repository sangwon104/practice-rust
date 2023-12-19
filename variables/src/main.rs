fn main() {
    // practice_variable();     
    // practice_function(5, 'y');
    // let statement = (let a = 1) // error
    // let expr = practice_expression();
    // practice_branches(3); practice_branches(2); practice_branches(4);
}

fn practice_branches(num: i32) {
    if num == 2 {
        println!("is 2");
    } else if num == 4 {
        println!("is 4");
    } else {
        println!("is etc");
    }
}

fn practice_expression() -> i32 {
    let x = 1 + 3;
    // return x; // success
    x // success (세미 콜론 생략)
    // x; // error
}

fn practice_function(x: i32, y: char) {
    println!("parameter : {x}, {y}");

}

fn practice_variable() {
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
