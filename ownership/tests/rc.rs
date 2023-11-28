use spectral::option::OptionAssertions;
use std::rc::Rc;

use spectral::assert_that;

#[test]
fn rc_two_owners_can_read() {
    let x = String::from("Hello");
    let reference_counted_string = Rc::new(x);
    let rc_2 = reference_counted_string.clone();
    println!(
        "reference_counted_string = {}, rc_2 = {}",
        reference_counted_string, rc_2
    );
}

#[test]
fn rc_cannot_mutate_shared_value() {
    let x = String::from("Hello");
    let reference_counted_string = Rc::new(x);
    let mut rc_2 = reference_counted_string.clone();

    println!(
        "reference_counted_string = {}, rc_2 = {}",
        reference_counted_string, rc_2
    );

    assert_that!(Rc::get_mut(&mut rc_2)).is_none();
}

#[test]
fn rc_can_mutate_unshared_value() {
    let x = String::from("Hello");
    let mut reference_counted_string = Rc::new(x);

    *Rc::get_mut(&mut reference_counted_string).unwrap() = String::from("Other String");
    assert_that!(reference_counted_string.as_ref().as_ref()).is_equal_to("Other String");
}
