use ownership::get_string_len;

fn main() {
    let a = String::from("I am a string");
    let a_len = get_string_len(a);
    //the value a is gone. We do not own it anymore.
    //It is now owned by the function get_string_len
    //Unfortunately that function does not give us the value back.
    //Thus we cannot use it anymore.
    println!("the string {} has lenght {}", a, a_len);
}
