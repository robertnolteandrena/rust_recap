use rand::{thread_rng, Rng};
use spectral::assert_that;
use trait_objects_lib::{Goat, HasName, Sheep, SmartZoo, Zoo};

#[test]
fn test_name_of_zoo() {
    let goat = Goat;
    let sheep = Sheep;
    let zoo = Zoo {
        animals: vec![&goat, &sheep],
    };
    let zoo_name = zoo.name();
    assert_that!(zoo_name.as_ref()).is_equal_to("Zoo of: Goat Sheep");
}

#[test]
fn dispatch_really_occurs_at_runtime() {
    let mut rng = thread_rng();
    let goat = Goat;
    let sheep = Sheep;
    let mut zoo = Zoo {
        animals: vec![&goat, &sheep],
    };
    let additional_goat: bool = rng.gen();
    let additional_animal: &'static dyn HasName = if additional_goat { &Goat } else { &Sheep };
    zoo.animals.push(additional_animal);

    let zoo_name = zoo.name();
    if additional_goat {
        assert_that!(zoo_name.as_ref()).is_equal_to("Zoo of: Goat Sheep Goat");
    } else {
        assert_that!(zoo_name.as_ref()).is_equal_to("Zoo of: Goat Sheep Sheep");
    }
}

#[test]
fn does_the_zoo_own_animals_quesionmark() {
    //We will assert that this file will not compile. We expect the compiler error message saved in
    //"tests/will_not_compile/the_zoo_does_not_own_animals.stderr"
    //Notice that your IDE will not give you any errors in that file. This is because the file is
    //not part of the project. If it were, our whole project would not compile. It is treated as a
    //standalone binary. (Thats also why it has the main function)
    //to answer the question: Does the zoo own animals ? No!

    let t = trybuild::TestCases::new();
    t.compile_fail("tests/will_not_compile/the_zoo_does_not_own_animals.rs")
}

#[test]
fn does_the_smart_zoo_own_animals_quesionmark() {
    let mut smart_zoo = SmartZoo { animals: vec![] };
    {
        let goat = Goat;
        smart_zoo.animals.push(Box::new(goat));
    }

    let zoo_name = smart_zoo.name();
    assert_that!(zoo_name.as_ref()).is_equal_to("SmartZoo of: Goat");
    //Well
    //The smart_zoo owns a Box smart pointer
    //That Smart Pointer owns the animal which is allocated on the heap
    //So yes the smart zoo owns its animals transitively
}
