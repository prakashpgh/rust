fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    } 
    &s[..]
}

//string literal as paramter, works with the whole string as well as slices
fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    } 
    &s[..]
}

pub fn slicing_demo()  {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    //compile error...
    //s.clear();
    println!("the first word is: {word}");

    //str
    let word_str = first_word_str(&s);
    println!("the first word_str is: {word_str}");

    let word_str_1 = first_word_str(&s[0..6]);
    println!("the first word_str_1 is: {word_str_1}");

}
