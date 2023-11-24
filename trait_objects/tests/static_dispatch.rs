use spectral::assert_that;
use trait_objects_lib::{BoringZoo, Goat, HasName};

#[test]
fn test_name_of_boring_zoo() {
    let goat = Goat;
    let boring_zoo = BoringZoo {
        animals: vec![goat],
    };
    let boring_zoo_name = boring_zoo.name();
    assert_that!(boring_zoo_name.as_ref()).is_equal_to("BoringZoo of: Goat");
}

#[test]
fn does_the_boring_zoo_own_animals_quesionmark() {
    let mut boring_zoo = BoringZoo { animals: vec![] };
    {
        let goat = Goat;
        boring_zoo.animals.push(goat);
    }

    let boring_zoo_name = boring_zoo.name();
    assert_that!(boring_zoo_name.as_ref()).is_equal_to("BoringZoo of: Goat");
    //Yes it does
}

#[test]
fn is_the_boring_zoo_boring_quesionmark() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/will_not_compile/the_boring_zoo_can_only_hold_one_type_of_animal.rs")
    //Yes it is
}
