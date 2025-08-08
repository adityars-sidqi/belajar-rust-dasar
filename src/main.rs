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

#[test]
fn static_typing() {
    let name = "Aditya";
    println!("Hello, {}!", name);

    // name = 10;
    println!("Hello, {}!", name);
}

#[test]
fn shadowing() {
    let name = "Aditya";
    println!("Hello, {}!", name);

    let name = 10;
    println!("Hello, {}!", name);
}

/*
    ini komentar lebih dari satu baris
    ini komentar lebih dari satu baris
    ini komentar lebih dari satu baris
 */
#[test]
fn comment() {
    // ini komentar
    println!("Hello"); //ini komentar lagi
}