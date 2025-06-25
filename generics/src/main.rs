mod generic_examples;
mod lifetimes;

fn main() {
    println!("generic examples");
    generic_examples::generic_demo();

    //trait..
    generic_examples::trait_demo();

    //lifetimes
    lifetimes::lifetimes_demo();
}
