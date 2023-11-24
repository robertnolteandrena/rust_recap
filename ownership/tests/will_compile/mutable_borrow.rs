#[allow(unused_mut)]
#[allow(dead_code)]
fn main() {
    let mut a = String::from("Hello");
    let mut b = &a;
    println!("{}, {}", a, b);
}
