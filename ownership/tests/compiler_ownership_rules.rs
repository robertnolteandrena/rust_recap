//uncomment the line below to see errors in IDE. The whole project will no longer compile.
//mod will_not_compile;

mod will_compile;
#[test]
fn ownership_enforced_by_compiler() {
    //in the subfolders will_compile and will_not_compile
    //are programs that adhere to the ownership rules (will_compile)
    //and programs that do not adhere to the ownership rules (will_not_compile)
    let t = trybuild::TestCases::new();
    t.pass("tests/will_compile/borrow_to_function.rs");
    t.pass("tests/will_compile/multiple_borrow.rs");

    t.compile_fail("tests/will_not_compile/loose_ownership_to_function.rs");
    t.compile_fail("tests/will_not_compile/value_must_have_an_owner.rs");
    t.compile_fail("tests/will_not_compile/value_must_not_have_more_than_one_owner.rs");
    t.compile_fail("tests/will_not_compile/multiple_mutable_borrow.rs");
    t.compile_fail("tests/will_not_compile/immutable_and_mutable_borrow.rs");
}
