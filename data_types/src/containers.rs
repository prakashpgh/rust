pub fn containers_demo() {
    println!("--------------------------");

    println!("containers demo");
    //create vector and push
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    //iterate
    for i in &v {
        println!("{i}");
    }

    //mutate...
    for i in &mut v {
        *i += 20;
    }

    for i in &v {
        println!("{i}");
    }

    //get element at an index.
    let second = &v[1];
    let test = &v[1];
    println!("{second}");
    println!("{test}");
    println!("second: {}", v[1]);

    //.get
    let op: Option<&i32> = v.get(1);
    match op {
        None => println!("the element does not exist"),
        Some(op) => println!("the number is {}", op),
    }

    //get returns Option<&i32>
    let yy = v.get(100);

    //this will panic since the vector does not have 99 elements
    //let zz = &v[100];//WILL PANIC

    let zz = &v[1];

    //mutable & immutable references. NOT allowed.
    let imm = &v[0];
    println!("{imm}");
    v.push(4);
    //below line causes error with v.push()  => mutable and immutable ref. are not allowed at the same time
    //why this behaves like this... if the push causes the vec to resize, v[0] would be invalid
    //println!("{imm}");

    //using vector & enum to store the data types of different types.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("--------------------------");
}

use std::collections::HashMap;
pub fn hashmap_demo() {
    println!("--------------------------");
    println!("HashMap");

    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("A"), 1);
    scores.insert(String::from("B"), 2);

    let key = String::from("A");
    let score = scores.get(&key).copied().unwrap_or(0);
    println!("{} : {}", key, score);

    println!("iterating k,v");
    for (key, val) in &scores {
        println!("{} : {}", key, val);
    }

    //values inserted into hashmap are owned by hashmap if the they dont have copy trait.. eg. String is owned by HM, while int is copied.
    let s = String::from("A1");
    println!("{}", s);
    let mut own_test: HashMap<String, String> = HashMap::new();
    own_test.insert(String::from("A"), s);
    //println!("{}", s);  //note this line gives a compile error... s have been owned by the hashmap
    for (key, val) in &own_test {
        println!("{} : {}", key, val);
    }

    println!("updating the value");
    //value is now chaned to S1
    let s1 = String::from("A3");
    own_test.insert(String::from("A"), s1);
    for (key, val) in &own_test {
        println!("{} : {}", key, val);
    }

    //value is now NOT changed to A4
    own_test
        .entry(String::from("A"))
        .or_insert(String::from("A4"));
    for (key, val) in &own_test {
        println!("{} : {}", key, val);
    }

    println!("--------------------------");
}
