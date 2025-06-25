mod containers;
mod strings;
mod structs;

fn main() {
    println!("structs...");
    //structs
    structs::structs_demo();

    //enum demo
    structs::enun_demo();

    //containers demo
    containers::containers_demo();

    //strings
    strings::strings_demo();

    //hashmap
    containers::hashmap_demo();

}
