use spectral::{assert_that, vec::VecAssertions};
use traits_lib::{LibraryGeneric, Task};

#[test]
fn add_element_of_same_type_string() {
    let mut library = LibraryGeneric { described: vec![] };
    library.add_element(String::from("hello"));
    library.add_element(String::from("bye"));
    assert_that!(library.described).has_length(2);
}

#[test]
fn add_element_of_same_type_task() {
    let mut library = LibraryGeneric { described: vec![] };
    library.add_element(Task {
        urgency: 1,
        name: String::from("Buy groceries"),
    });
    library.add_element(Task {
        urgency: 255,
        name: String::from("Drink more water"),
    });
    assert_that!(library.described).has_length(2);
}

#[test]
fn add_element_of_different_type() {
    //This does not work
    /*
    let mut library = LibraryGeneric { described: vec![] };
    library.add_element(String::from("Hello"));
    library.add_element(Task {
        urgency: 255,
        name: String::from("Drink more water"),
    });
    assert_that!(library.described).has_length(2);
    */
}
