#![allow(non_snake_case)]

use spectral::assert_that;

fn get_first_char(data: &str) -> Option<char> {
    data.chars().next()
}

/*
#[test]
fn cannot_index_string() {
    let hello_world = String::from("Привіт світ");
    hello_world[1];
    //the type std::string::String cannot be indexed by `{integer}`
}
 */

#[test]
fn representing_utf8() {
    let hello_world = String::from("Привіт світ");

    let num_bytes = hello_world.len();
    let num_chars = hello_world.chars().count();
    assert_that!(num_chars).is_equal_to(11);
    assert_that!(num_bytes).is_equal_to(21);
}

#[test]
fn call_with_str_reference() {
    let data = "hello";
    let number_of_chars = get_first_char(data);
    assert_that!(number_of_chars).is_equal_to(Some('h'));
}

#[test]
fn call_with_string_reference() {
    let data = String::from("hello");
    let number_of_chars = get_first_char(&data);
    assert_that!(number_of_chars).is_equal_to(Some('h'));
}

#[test]
fn repeated_deref_coercion() {
    let data = Box::new(String::from("hello"));
    let number_of_chars = get_first_char(&data);
    assert_that!(number_of_chars).is_equal_to(Some('h'));
}
