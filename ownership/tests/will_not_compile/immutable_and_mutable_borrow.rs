fn main() {
    let mut a = String::from("Hello");
    let mutable_borrow = &mut a;
    let immutable_borrow = &a;
    println!("{} {}", mutable_borrow, immutable_borrow);
}
