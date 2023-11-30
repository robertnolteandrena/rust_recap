use std::usize;

use spectral::assert_that;

fn get_number_of_chars(data: &str) -> usize {
    data.chars().count()
}

#[test]
fn call_with_str_reference() {
    let data = "hello";
    let number_of_chars = get_number_of_chars(data);
    assert_that!(number_of_chars).is_equal_to(5);
}

#[test]
fn call_with_string_reference() {
    let data = String::from("hello");
    let number_of_chars = get_number_of_chars(&data);
    assert_that!(number_of_chars).is_equal_to(5);
}

#[test]
fn repeated_deref_coercion() {
    let data = Box::new(String::from("hello"));
    let number_of_chars = get_number_of_chars(&data);
    assert_that!(number_of_chars).is_equal_to(5);
}
