#[allow(dead_code)]
fn main() {
    let a = String::from("Hello");
    let immutable_borrow_1 = &a;
    let immutable_borrow_2 = &a;
    let _result = format!("{}, {}", immutable_borrow_1, immutable_borrow_2);
}
