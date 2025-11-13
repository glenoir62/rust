use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.name, self.age)
    }
}

fn main() -> std::io::Result<()> {
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
    };

    // Sérialiser et écrire dans un fichier JSON
    let encoded = serde_json::to_string(&person)?;
    let mut file = File::create("person.json")?;
    file.write_all(encoded.as_bytes())?;

    // Lire et désérialiser à partir du fichier JSON
    let mut file = File::open("person.json")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let decoded: Person = serde_json::from_str(&buffer)?;
    println!("{decoded}");

    Ok(())
}