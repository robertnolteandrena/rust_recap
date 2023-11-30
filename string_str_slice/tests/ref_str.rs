use spectral::assert_that;

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
fn string_reference_does_not_live_forever() {
    let a: &String;
    {
        let string_reference: &String = &String::from("Hello");
        a = string_reference;
    }
    assert_that!(a.as_str()).is_equal_to("Hello");
} */
