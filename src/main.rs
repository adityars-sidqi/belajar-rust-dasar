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

#[test]
fn explicit() {
    let age : i32 = 20;
    println!("{}", age);
}

#[test]
fn number() {
    let a : i32 = 10;
    println!("{}", a);

    let b: f64 = 10.5;
    println!("{}", b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    let d: i64 = 1000000000;
    let e: i8 = d as i8;
    println!("{}", e);
}

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 10;
    let c = a * b;
    println!("{}", c);
    let d = a / b;
    println!("{}", d);
    let e = a + b;
    println!("{}", e);
}

#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

#[test]
fn comparison_operators() {
    let result: bool = 20 >= 20;
    println!("{}", result);
}

#[test]
fn boolean_operators() {
    let a = true;
    let b = false;

    let result = a && b;
    println!("{}", result);
}