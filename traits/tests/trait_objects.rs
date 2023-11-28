use spectral::{assert_that, vec::VecAssertions};
use traits_lib::{LibraryTraitObject, Task};

#[test]
fn add_element_of_same_type_string() {
    let mut library = LibraryTraitObject { described: vec![] };
    library.add_element(Box::new(String::from("hello")));
    library.add_element(Box::new(String::from("bye")));
    assert_that!(library.described).has_length(2);
}

#[test]
fn add_element_of_same_type_task() {
    let mut library = LibraryTraitObject { described: vec![] };
    library.add_element(Box::new(Task {
        urgency: 1,
        name: String::from("Buy groceries"),
    }));
    library.add_element(Box::new(Task {
        urgency: 255,
        name: String::from("Drink more water"),
    }));
    assert_that!(library.described).has_length(2);
}

#[test]
fn add_element_of_different_type() {
    let mut library = LibraryTraitObject { described: vec![] };
    library.add_element(Box::new(String::from("Hello")));
    library.add_element(Box::new(Task {
        urgency: 255,
        name: String::from("Drink more water"),
    }));
    assert_that!(library.described).has_length(2);
}
