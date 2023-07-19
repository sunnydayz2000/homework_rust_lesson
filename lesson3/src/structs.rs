use serde::{Serialize, Deserialize};
use crate::crypto::sha256;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Role {
    Student,
    Teacher,
    Principal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub  id: String,
    pub  name: String,
    pub  password: String,
    pub  role: Role,
}

impl User {
    pub fn authenticate(&self, password: &str) -> bool {
        self.password == sha256(password)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
    pub id: String,
    pub name: String,
    pub description: String,
    pub teacher_id: String,
    pub classroom_id: String,
    pub grades: std::collections::HashMap<String, f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Classroom {
    pub id: String,
    pub course_ids: Vec<String>,
}