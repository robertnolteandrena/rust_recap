fn main() {
    let b;
    {
        let a = String::from("Hello World!");
        b = &a;
        //a goes out of scope here
        //the value no longer has an owner
    }
    println!("b = {b}"); //The value is invalid
}
