use add;
use multiply;

fn main() {
    println!("Hello, world!");
    let i = 6;
    let j = 7;

    let result = multiply::multiply(i, j);
    println!("multiply: {}", result);

    let result2 = add::add(i, j);
    println!("add: {}", result2);
}
