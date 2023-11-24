use ownership::get_string_len_borrowed;

#[allow(dead_code)]
fn borrow_to_function() {
    let a = String::from("I am a string");
    let a_len = get_string_len_borrowed(&a);
    println!("the string {} has lenght {}", a, a_len);
}
#[allow(dead_code)]
fn main() {}
