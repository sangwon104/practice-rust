fn main() {
    // practice_ownership();
    // practice_reference_borrow();
    // practice_mutable_reference();
    // practice_scope_reference();
    // practice_dangling_pointer();
    practice_slice();
}

fn practice_slice() {
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello}");
    println!("{world}");
    
    let start = &s[..2];
    let end = &s[2..];
    println!("{start}");
    println!("{end}");

    let first = first_word(&s);
    println!("{first}");

    let second = second_word(&s);
    println!("{second}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..4];
    assert_eq!(slice, &[2, 3, 4]);
}

fn first_word (s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word (s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[(i+1)..];
        }
    }

    &s[..]
}

fn practice_dangling_pointer() {
    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
    println!("{reference_to_nothing}");
}

fn no_dangle() -> String {
    let str = String::from("hello");
    str
}

// fn dangle() -> &String {
//     let str = String::from("hello");
//     &str
// }

fn practice_scope_reference() {
    let mut str = String::from("hello");

    let ref1 = &str;
    let ref2 = &str;
    println!("{ref1}, {ref2}");

    let ref3 = &mut str;
    println!("{ref3}");
}

fn practice_mutable_reference() {
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