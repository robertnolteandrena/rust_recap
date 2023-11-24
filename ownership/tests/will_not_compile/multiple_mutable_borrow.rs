fn main() {
    let mut a = String::from("Hello");
    let mutable_borrow_1 = &mut a;
    let mutable_borrow_2 = &mut a;
    println!("{}, {}", mutable_borrow_1, mutable_borrow_2);
}
