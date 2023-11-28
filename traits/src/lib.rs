pub struct Sheep;
pub struct Goat;

pub trait HasName {
    fn name(&self) -> String;
}

impl HasName for Goat {
    fn name(&self) -> String {
        "Goat".to_owned()
    }
}
impl HasName for Sheep {
    fn name(&self) -> String {
        "Sheep".to_owned()
    }
}

pub struct Zoo<'a> {
    pub animals: Vec<&'a dyn HasName>,
}
pub struct SmartZoo {
    pub animals: Vec<Box<dyn HasName>>,
}
pub struct BoringZoo<T: HasName> {
    pub animals: Vec<T>,
}

impl<'a> HasName for Zoo<'a> {
    fn name(&self) -> String {
        let animals = self
            .animals
            .iter()
            .map(|animal| animal.name())
            .collect::<Vec<String>>()
            .join(" ");
        ["Zoo of: ", &animals].concat()
    }
}
impl HasName for SmartZoo {
    fn name(&self) -> String {
        let animals = self
            .animals
            .iter()
            .map(|animal| animal.name())
            .collect::<Vec<String>>()
            .join(" ");
        ["SmartZoo of: ", &animals].concat()
    }
}
impl<T: HasName> HasName for BoringZoo<T> {
    fn name(&self) -> String {
        let animals = self
            .animals
            .iter()
            .map(|animal| animal.name())
            .collect::<Vec<String>>()
            .join(" ");
        ["BoringZoo of: ", &animals].concat()
    }
}

impl HasName for String {
    fn name(&self) -> String {
        self.clone()
    }
}
