#[derive(PartialEq, Clone, serde::Serialize, serde::Deserialize, Debug)]

pub struct Project {
    pub name: String,
    pub description: String,
}

impl Project {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }
}
