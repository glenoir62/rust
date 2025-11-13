use csv::{ReaderBuilder, WriterBuilder};
use crate::models::Person;

/// Lit un fichier CSV, modifie des données et réécrit le fichier
pub fn update_csv<F>(input_filename: &str, output_filename: &str, modifier: F) -> Result<(), csv::Error>
where
    F: Fn(&mut Vec<Person>),
{
    println!("\n=== Mise à jour du fichier: {} -> {} ===", input_filename, output_filename);
    
    // Lire le fichier CSV
    let file = std::fs::File::open(input_filename)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    
    // Collecter les enregistrements dans un vecteur de structures Person
    let mut people: Vec<Person> = rdr
        .deserialize()
        .filter_map(|result| result.ok())
        .collect();
    
    println!("✓ {} personnes lues", people.len());
    
    // Appliquer la modification
    modifier(&mut people);
    
    // Réécrire le fichier CSV avec les données mises à jour
    let output_file = std::fs::File::create(output_filename)?;
    let mut wtr = WriterBuilder::new().from_writer(output_file);
    
    // Écrire chaque enregistrement dans le fichier CSV
    for person in &people {
        wtr.serialize(person)?;
    }
    
    wtr.flush()?;
    println!("✓ {} personnes écrites dans {}", people.len(), output_filename);
    
    Ok(())
}

/// Met à jour l'âge d'une personne par son nom
pub fn update_age_by_name(filename: &str, name: &str, new_age: u32) -> Result<(), csv::Error> {
    update_csv(filename, filename, |people| {
        if let Some(person) = people.iter_mut().find(|p| p.name == name) {
            println!("✓ Mise à jour: {} ({} -> {} ans)", person.name, person.age, new_age);
            person.age = new_age;
        } else {
            println!("⚠ Personne non trouvée: {}", name);
        }
    })
}

/// Supprime une personne par son nom
pub fn delete_by_name(filename: &str, name: &str) -> Result<(), csv::Error> {
    update_csv(filename, filename, |people| {
        let initial_count = people.len();
        people.retain(|p| p.name != name);
        let removed = initial_count - people.len();
        if removed > 0 {
            println!("✓ {} personne(s) supprimée(s)", removed);
        } else {
            println!("⚠ Personne non trouvée: {}", name);
        }
    })
}
