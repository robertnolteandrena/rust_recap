pub trait HasDescription {
    fn description(&self) -> String;
}

pub struct Task {
    pub urgency: u8,
    pub name: String,
}

impl HasDescription for String {
    fn description(&self) -> String {
        format!("Description: {}", self)
    }
}

impl HasDescription for Task {
    fn description(&self) -> String {
        format!("Task {} has urgency {}.", self.name, self.urgency)
    }
}
pub struct LibraryTraitObject {
    pub described: Vec<Box<dyn HasDescription>>,
}
pub struct LibraryGeneric<T: HasDescription> {
    pub described: Vec<T>,
}

impl<T: HasDescription> LibraryGeneric<T> {
    pub fn add_element(&mut self, t: T) {
        self.described.push(t);
    }
    pub fn describe_all(&self) {
        for element in &self.described {
            println!("{}", element.description());
        }
    }
}

impl LibraryTraitObject {
    pub fn add_element(&mut self, t: Box<dyn HasDescription>) {
        self.described.push(t);
    }
    pub fn describe_all(&self) {
        for element in &self.described {
            println!("{}", element.description());
        }
    }
}
