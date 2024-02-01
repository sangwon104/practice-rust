fn main() {
    // practice_ownership();
    // practice_reference_borrow();
    practice_mutable_reference();
}

fn practice_mutable_reference(){
    let mut s1 = String::from("hello");
    println!("s1 : {}", s1);
    change(&mut s1);
    println!("s1 : {}", s1);
}

fn change(str: &mut String){
    str.push_str(", world");
}

fn practice_reference_borrow() {
    let s1 = String::from("hello");
    // asis
    // let (s2, length) = calculate_length_1(s1);
    // println!("s2 : {}, length : {}", s2, length);
    // tobe
    let length = calculate_length_2(&s1);
    println!("s1 : {}, length : {}", s1, length)
}

fn calculate_length_1(str: String) -> (String, usize) {
    let length = str.len();
    // (str, str.len() // moved error
    (str, length)
}

fn calculate_length_2(str: &String) -> usize {
    str.len()
}

fn practice_ownership() {
    // happen_moved_error();
    // practice_deep_copy();
    
    // ownership
    let s1 = gives_ownership(); // s1 drop 
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 drop
    println!("s3 : {}", s3);
} // s3 drop

fn gives_ownership() -> String {
    let str = String::from("hi");
    str // str drop
}

fn takes_and_gives_back(some_thing: String) -> String {
    some_thing // some_thing drop
}