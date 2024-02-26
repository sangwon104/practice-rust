fn main() {
    // vector_basic();
    vector_enum();
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
