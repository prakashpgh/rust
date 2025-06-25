//Borrow checker

//below code does not compile... => expected named lifetime paramter
//fn longest(x: &str, y: &str) -> &str {
//   if x.len() > y.len() { x } else { y }
//}//
///

//lifetime of the return => same as the lifetime of the parameters
//overlap / smaller of the lifetimes of the 2 parameters
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

pub fn lifetimes_demo() {
    println!("lifetimes_demo..");
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("{}", result);

    //below code wont compile.
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");
}

//a struct can also hold references, here also the lifetimes can be specified

//lifetime parameters need to be specified for functions / structs that use them

//lifetime elison rules => lifetime elison rules that are built into compiler.. and do not need to be specified.
//functions
//input lifetimes => lifetimes on input parameters
//output lifetimes => lifetimes on output
//Rules
//rule 1 => The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
//fn foo<'a>(x: &'a i32)
//fn foo<'a, 'b>(x: &'a i32, y: &'b i32)

//rule 2 => if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32

//rule 3 => The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters

//if after applying all 3 rules, compiler is not able to apply the lifetime parameter to the output, then it gives compile error.

/*
Lifetime in method definitions

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

*/

/*
static lifetime  => live for entire duration of the program
let s: &'static str = "I have a static lifetime.";

*/

//lifetimes are a type of generic
/*
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
*/
