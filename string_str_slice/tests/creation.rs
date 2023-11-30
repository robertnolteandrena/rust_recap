use spectral::assert_that;

#[allow(non_snake_case)]
#[test]
fn filling_string() {
    let str_reference: &str = "Hello";
    let _String: String = str_reference.to_string();

    //The type of str_reference is &str, while the type of _String is String.
    //If this would not be the case, cargo test would fail, because we are explicitly setting the
    //types.
}

#[test]
fn a_literal_str_is_valid_for_the_lifetime_of_the_program() {
    let a: &str;
    {
        let str_reference: &str = "I will live forever";
        a = str_reference;
    }
    assert_that!(a).is_equal_to("I will live forever");
}
/*
#[allow(non_snake_case)]
#[test]
fn the_same_with_some_other_type_does_not_work() {
    let a: &u32;
    {
        let u32_owned: u32 = 101;
        let u32_reference: &u32 = &u32_owned;
        a = u32_reference;
    }
    assert_that!(a).is_equal_to(&101);
} */
