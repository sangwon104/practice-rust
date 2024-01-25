fn main() {
    // practice_variable();     
    // practice_function(5, 'y');
    // let statement = (let a = 1) // error
    // let expr = practice_expression();
    // practice_branches(3); practice_branches(2); practice_branches(4);
    // practice_loop(3);
    // practice_loop_label();
    // practice_while();
    // practice_for();
    practice_for_range();
}

fn practice_for_range() {
    println!("range print");
    for el in (1..10) {
        println!("element : {el}");
    }

    println!("range print reverse");
    for el in (1..10).rev() {
        println!("element : {el}");
    }
}

fn practice_for() {
    let arr = [1, 2, 3, 4];

    for el in arr {
        println!("arr : {el}");
    }
}

fn practice_while() {
    let mut index = 0;
    while index < 10 {
        println!("index : {index}");
        index += 1;
    }
}

fn practice_loop_label() {
    let mut count: i32 = 0;

    'counting_up: loop {
        
        println!("count : {count}");
        let mut remaining = 10;
        loop {
            println!("remaining : {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }


        count += 1;
    }
    println!("count of end : {count}");
}

fn practice_loop(loop_count: i32) {
    let mut current_count = 0;
    loop {
        println!("hello loop : {current_count}");
        if current_count > loop_count {
            break println!("break count!");
        }
        current_count += 1;
    }
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
