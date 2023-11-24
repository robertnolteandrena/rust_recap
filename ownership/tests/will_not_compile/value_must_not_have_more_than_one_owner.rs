fn main() {
    let a = String::from("Hello World!");
    let b = a; //variable b now owns the value
    println!("a = {a}"); //a is no longer valid
}
