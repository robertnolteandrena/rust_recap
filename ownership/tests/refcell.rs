use std::cell::RefCell;

use spectral::assert_that;

fn interior_mutability(refcell: &RefCell<String>) {
    //refcell is not declared mutable
    *refcell.borrow_mut() = String::from("bye");
}
#[test]
fn test_interior_mutability() {
    let refcell = RefCell::new(String::from("Hello"));
    interior_mutability(&refcell);
    assert_that!(refcell.borrow_mut().as_ref()).is_equal_to("bye");
}
