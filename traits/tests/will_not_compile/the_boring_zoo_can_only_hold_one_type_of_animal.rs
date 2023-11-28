use spectral::assert_that;
use traits_lib::{BoringZoo, Goat, HasName, Sheep};

fn boring_zoo_can_only_hold_one_type_of_animal() {
    let goat = Goat;
    let sheep = Sheep;
    let boring_zoo = BoringZoo {
        animals: vec![goat, sheep],
    };
    let boring_zoo_name = boring_zoo.name();
    assert_that!(boring_zoo_name.as_ref()).is_equal_to("BoringZoo of: Goat Sheep");
}
fn main() {}
