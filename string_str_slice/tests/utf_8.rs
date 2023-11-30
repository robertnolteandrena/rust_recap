use spectral::assert_that;

#[test]
fn representing_utf8() {
    let hello_world = String::from("Привіт світ.");

    let num_bytes = hello_world.len();
    let num_chars = hello_world.chars().count();
    assert_that!(num_chars).is_equal_to(12);
    assert_that!(num_bytes).is_equal_to(22);
    // The String type wraps a Vec<u8>, however utf8 characters can have more than a single byte.
}
/*
#[test]
fn cannot_index_string() {
    let hello_world = String::from("Привіт світ.");
    hello_world[1];
    //the type std::string::String cannot be indexed by `{integer}`
}
 */

#[test]
fn slicing_utf8() {
    let hello_world = "Привіт світ.";
    let str_slice: &str = &hello_world[0..4];
    assert_that!(str_slice).is_equal_to("Пр")
}

#[test]
#[should_panic]
fn cannot_slice_within_character() {
    let hello_world = "Привіт світ.";
    let _str_slice: &str = &hello_world[0..3];
}
