use spectral::assert_that;
use traits_lib::{Goat, HasName, Zoo};

fn does_the_zoo_own_animals_quesionmark() {
    let mut zoo = Zoo { animals: vec![] };
    {
        let goat = Goat;
        zoo.animals.push(&goat);
    }

    let zoo_name = zoo.name();
    assert_that!(zoo_name.as_ref()).is_equal_to("Zoo of: Goat");
}
fn main() {}
