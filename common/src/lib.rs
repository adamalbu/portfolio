use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub description: String,
}

impl Project {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
}
