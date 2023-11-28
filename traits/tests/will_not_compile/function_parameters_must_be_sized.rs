use rand::{thread_rng, Rng};
use spectral::assert_that;
use traits_lib::{Goat, HasName, Sheep};
fn test_function<T: HasName>(t: &T) -> String {
    format!("this is test_function: {}", t.name())
}
fn main() {
    let mut rng = thread_rng();
    let is_goat = rng.gen();
    let name_haver: &dyn HasName = if is_goat { &Goat } else { &Sheep };
    let test_string = test_function(name_haver);
    if is_goat {
        assert_that!(test_string.as_ref()).is_equal_to("this is test_function Goat");
    } else {
        assert_that!(test_string.as_ref()).is_equal_to("this is test_function Sheep");
    }
}
