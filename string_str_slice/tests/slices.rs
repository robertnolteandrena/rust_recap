use spectral::assert_that;

#[test]
fn taking_a_slice_from_an_array() {
    let data = [0, 1, 2, 3, 4];
    let slice: &[i32] = &data[1..2];
    assert_that!(slice).is_equal_to([1].as_slice())
}

#[test]
fn taking_a_slice_from_a_string() {
    let hello_world = "hello world";
    let hello = &hello_world[..5];
    let world = &hello_world[6..];
    assert_that!(hello).is_equal_to("hello");
    assert_that!(world).is_equal_to("world");
}
#[test]
fn slicing_utf8() {
    let hello_world = "Привіт світ";
    let str_slice: &str = &hello_world[0..4];
    assert_that!(str_slice).is_equal_to("Пр")
}

#[test]
#[should_panic]
fn cannot_slice_within_character() {
    let hello_world = "Привіт світ";
    let _str_slice: &str = &hello_world[0..3];
}
