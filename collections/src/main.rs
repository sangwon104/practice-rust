fn main() {
    // vector_basic();
    // vector_enum();
    // string_basic();
    // string_format_macro();
    string_indexing();
}

fn string_indexing() {
    let s1 = String::from("hello");
    let h = s1[0];
    println!("h : {}", h);
}

fn string_format_macro() {
    let s1 = String::from("hello, ");
    let s2 = String::from("world.");
    let s3 = s1 + &s2;
    println!("s3 : {}", s3);

    let st1 = String::from("tic");
    let st2 = String::from("tac");
    let st3 = String::from("toe");
    // let result = st1 + "-" + &st2 + "-" + &st3;
    // println!("result : {}", result);

    let result = format!("{st1}-{st2}-{st3}");
    println!("result : {}", result);
}

fn string_basic() {
    let data = "initial contents";
    let s1 = data.to_string();
    let s2 = String::from("initial contents"); // "initial contents".to_string() 와 동일한 의미
    println!("data : {}", data);
    println!("s1 : {}", s1);
    println!("s2 : {}", s2);

    let mut s3 = String::from("hello");
    s3.push_str("hi");
    println!("s3 : {}", s3);
    s3.push('s');
    println!("s3 : {}", s3);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn vector_enum() {
    let row = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Float(2.45),
        SpreadsheetCell::Text(String::from("hello"))
    ];

    println!("SpreadsheetCell : {:#?}", row);
}

fn vector_basic() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(4);
    v1.push(5);
    println!("v1 : {:?}", v1);
    
    for i in &mut v1 {
        *i += 50;
    }

    let v1_third: &i32 = &v1[2]; 
    println!("v1 third : {}", v1_third);

    let v2 = vec![1, 2, 3, 4, 5];
    println!("v2 : {:?}", v2);

    for i in &v2 {
        println!("{i}");
    } 

    let v2_third: Option<&i32> = v2.get(2);
    match v2_third {
        Some(v2_third) => println!("v2 third : {}", v2_third),
        None => println!("Threr is no third element")
    }


}
