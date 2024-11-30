use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    pub id: u32,
    pub name: String,
    pub objects: Vec<String>,
}

impl Scene {
    pub fn new(id: u32, name: String, objects: Vec<String>) -> Self {
        Scene { id, name, objects }
    }

    pub fn add_object(&mut self, object: String) {
        self.objects.push(object);
    }
}
