mod ownership;
mod references;
mod slicing;



fn main() {
    println!("ownership...");
    ownership::owner_variable();


    ownership::owner_pass_to_func();


    ownership::owner_return_from_func();

    //references
    references::references_demo();

    //slicing
    slicing::slicing_demo();
}
