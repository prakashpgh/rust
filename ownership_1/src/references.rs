//passing to function as read-only reference
fn length(s : &String) -> usize {
    s.len()
}

//function with mutable arguments.
fn change_something(s1 : &mut String) {
    s1.push_str("pushed..");
}


fn multiple_muts() {
    println!("multiple muts..");
    let mut s1 = String::from("hello..");
    let r1 = &mut s1;
    println!("{}", r1);

    //2nd mutable reference is not allowed...
    //let r2 = &mut s1;
    //println!("{}, {}", r1, r2);

    //multiple live read-write not allowed 
    let mut sr = String::from("hello..sr");
    let r1 = & sr;
    println!("{}", r1);

    let r2 = & sr;
    println!("{}", r2);

    //let r3 = &mut sr;
    //println!("{}", r3);
    //println!("{}-{}--{}", r1, r2, r3);

}



pub fn references_demo() {
    println!("references_demo");
    let s1 = String::from("hello..");
    let l = length(&s1);
    println!("length: {l}");


    //write ref.
    let mut s1 = String::from("hello..");;
    change_something(&mut s1);
    println!("after change: {s1}");

    //multiple_muts
    multiple_muts();

}








