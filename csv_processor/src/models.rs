use serde::{Deserialize, Serialize};

/// Structure représentant une personne
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub city: String,
}

impl Person {
    /// Crée une nouvelle personne
    pub fn new(name: String, age: u32, city: String) -> Self {
        Person { name, age, city }
    }
    
    /// Affiche les informations de la personne
    pub fn display(&self) {
        println!("{} ({} ans) - {}", self.name, self.age, self.city);
    }
}
