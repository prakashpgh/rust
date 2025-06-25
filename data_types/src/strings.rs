pub fn strings_demo() {
    println!("--------------------------");
    println!("strings demo");
    let s = String::from("Rustaceans 😊");

    // Iterate over bytes
    println!("Bytes:");
    for b in s.bytes() {
        print!("{} ", b);
    }
    println!();

    //iterate chars..
    println!("chars:");
    for c in s.chars() {
        print!("{}", c);
    }
    println!();

    //index.
    println!("{}", &s[0..4]);

    //compile error 
    //println!("{}", &s[10..2]);

    
    //println!("{}", &hello[1]);

    let n = String::from("नमस्ते");
    println!("chars:");
    for c in n.chars() {
        print!("{}", c);
    }

    println!("bytes:");
    for b in n.bytes() {
        print!("{} ", b);
    }
    println!();
    //println!("{}", n);

    println!("--------------------------");
}
