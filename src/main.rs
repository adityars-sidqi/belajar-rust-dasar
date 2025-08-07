fn main() {
    println!("Hello, world!");

    println!("Hello, Aditya!");
}

#[test]
fn hello_test() {
    println!("Hello, Test!");
}

#[test]
fn test_variable() {
    let name = "Aditya";
    println!("Hello, {}!", name);
}

#[test]
fn test_mutable() {
    let mut name = "Aditya";
    println!("Hello, {}!", name);

    name = "Rahman";
    println!("Hello, {}!", name);
}