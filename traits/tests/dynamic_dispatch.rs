use rand::{thread_rng, Rng};
use spectral::{assert_that, string::StrAssertions};
use traits_lib::{Goat, HasName, Sheep, SmartZoo, Zoo};

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
    t.compile_fail("tests/will_not_compile/the_zoo_does_not_own_animals.rs");
    t.compile_fail("tests/will_not_compile/function_parameters_must_be_sized.rs");
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
fn test_function(t: &dyn HasName) -> String {
    format!("this is test_function: {}", t.name())
}

#[test]
fn dynamic_dispatch_function() {
    let mut rng = thread_rng();
    let is_goat = rng.gen();
    let name_haver: &dyn HasName = if is_goat { &Goat } else { &Sheep };
    let test_string = test_function(name_haver);
    if is_goat {
        assert_that!(test_string.as_ref()).is_equal_to("this is test_function: Goat");
    } else {
        assert_that!(test_string.as_ref()).is_equal_to("this is test_function: Sheep");
    }
}
#[test]
fn dynamic_dispatch_function_boxed() {
    let mut rng = thread_rng();
    let is_goat = rng.gen();
    let name_haver: Box<dyn HasName> = if is_goat {
        Box::new(Goat)
    } else {
        let number = rng.gen_range(0..1000);
        Box::new(format!("#{}", number))
    };
    let test_string = test_function(name_haver.as_ref());
    if is_goat {
        assert_that!(test_string.as_ref()).is_equal_to("this is test_function: Goat");
    } else {
        assert_that!(test_string.as_ref()).starts_with("this is test_function: #");
    }
}
