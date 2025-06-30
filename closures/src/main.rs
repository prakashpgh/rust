/*
closures are
    anonymous functions,
    can be stored in a variable,
    passed as parameter
    returned
    capture values from the env.

    somewhat equivalent to lambda in c++



    */

fn closure_examples() {
    let cls = || {
        println!("hello closure");
    };
    cls();

    //closure without braces {}
    let cls_2 = || println!("closure without braces..");
    cls_2();

    let cls_params = |a, b| a + b;
    let ret = cls_params(3, 4);
    println!("ret: {}", ret);

    //specifying parameters is not required, but allowed.
    //rust infers the parameter type from the parameters.
    //locked in argument types => once inferred the parameter types are locked in & cannot be changed
    //below code will give compile error.
    //println!("{}", cls_params(4.5, 3));

    //Case I => value capture..=> greet_base has been captured as immutable reference
    let greet_base = String::from("hello ");
    let greet = |name| println!("{} {}", greet_base, name);
    greet("jill");
    println!("{}", greet_base);

    //Case II => mutable closure
    let mut sum = 0;
    println!("sum: {}", sum);
    let mut closure_mutable = || sum += 1;
    //below println fails compilation => because mutable and immutable ref to sum cannot co-exist
    //println!("sum: {}", sum);
    closure_mutable();
    println!("sum: {}", sum);
    //closure_mutable();

    //Case III => using move for the closure to take ownership
    let mut str = String::from("created outside closlure...");
    println!("printing before closure {}", str);
    let closure_ownership = move || {
        println!("printing inside closure {}", str);
    };

    //below line gives compile error... since the closure took over the ownership of str and has gone out of scope
    //println!("printing after closure {}", str);
}

/*
Fn: trait that does not mutate the captured environment ~ immutable or does not change anything, can be called multiple times
FnMut: trait that mutates the captured env, can be called multiple times
FnOnce: can be called only once
*/

// This function takes an `Fn` closure.
// It can be called multiple times and cannot modify its captured environment.
fn apply_fn<F>(f: F)
where
    F: Fn(), // F must implement the Fn trait
{
    println!("Applying Fn closure:");
    f();
    f(); // Can call multiple times
}

// This function takes an `FnMut` closure.
// It can be called multiple times and can modify its captured environment.
fn apply_fn_mut<F>(mut f: F)
// `f` needs to be `mut` if the closure mutates its environment
where
    F: FnMut(), // F must implement the FnMut trait
{
    println!("Applying FnMut closure:");
    f();
    f(); // Can call multiple times
}

// This function takes an `FnOnce` closure.
// It can only be called once because it might consume captured values.
fn apply_fn_once<F>(f: F)
where
    F: FnOnce(), // F must implement the FnOnce trait
{
    println!("Applying FnOnce closure:");
    f(); // Called once
    // f(); // Cannot call again because it might have consumed values
}

fn functions_accepting_closures() {
    let i = 10;
    //case I: immut closure
    let closure_immut = || {
        println!("immut closure: {}", i);
    };
    apply_fn(closure_immut);

    //case II: mut closure
    let mut counter = 0;
    let closure_mutable = || {
        counter += 1;
        println!("counter: {}", counter);
    };
    apply_fn_mut(closure_mutable);

    //case III:
    let data = String::from("Rust");
    let closure_fn_once = move || {
        // Captures `data` by value (moves it)
        println!("Data in FnOnce: {}", data);
    };
    apply_fn_once(closure_fn_once);
}

//map
//filter
///for_each
fn closures_and_iterators() {
    //map...applied to each member of the vector
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);

    //number to string
    let str: Vec<String> = numbers.iter().map(|x| x.to_string()).collect();
    println!("string: {:?}", str);

    let even: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();
    println!("even: {:?}", even);

    //filter
    let min_value = 5;
    let filtered: Vec<i32> = even.into_iter().filter(|x| *x > min_value).collect();
    println!("filtered: {:?}", filtered);

    //for_each
    let names = vec!["Alice", "Bob", "Charlie"];

    // Print each name with a prefix
    let prefix = "User: ";
    names.iter().for_each(|name| {
        println!("{}{}", prefix, name); // `prefix` is captured by &T
    });

    //mutable..for_each
    let mut total_length = 0;
    names.iter().for_each(|name| {
        total_length += name.len(); // `total_length` is captured by &mut T
    });
    println!("Total length of names: {}", total_length);
}

fn main() {
    println!("closures");

    closure_examples();

    functions_accepting_closures();

    //usage of closures in iterators
    closures_and_iterators();
}
