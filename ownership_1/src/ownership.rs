




pub fn owner_variable() {
    println!("owner_variable...");
    let mut s1 = String::from("testing..");
    s1 += " appended";
    //pass ownership to other variable...
    let mut s2 = s1;

    //this should give compile error.
    //println!("{s1}");

    println!("{s2}");

    println!("----------------");
}


fn pass_to_func(s : String) {
    println!("{s}");

}


pub fn owner_pass_to_func() {
    println!("owner_pass_to_func...");

    let s = String::from("test...");
    
    //pass s to a function..
    pass_to_func(s);
    //try to access s, gives compile error.
    //println!("{s}");
    println!("----------------");
}




fn return_from_func(s : String) -> String {
    println!("return_from_func {s}");
    let mut s1:String = s;
    s1 += " return_from_func_";
    println!("----------------");
    s1
}


pub fn owner_return_from_func() {
    println!("owner_return_from_func...");

    let s = String::from("test...");
    
    //pass s to a function..
    let s2 = return_from_func(s);
    //try to access s, gives compile error.
    //println!("{s}");

    println!("owner_return_from_func... {}", s2);
    println!("----------------");
}

