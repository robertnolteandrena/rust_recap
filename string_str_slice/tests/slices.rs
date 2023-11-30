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
