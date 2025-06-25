mod exceptions;

fn main() {
    println!("exception handingl!");

    //below line calls panic..
    //exceptions::panic_demo();

    exceptions::vector_panic_demo();

    exceptions::file_error();

    exceptions::unwrap_else();

    exceptions::error_handling_using_match();

    exceptions::using_result();
}
