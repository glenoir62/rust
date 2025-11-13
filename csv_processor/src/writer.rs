use csv::WriterBuilder;
use crate::models::Person;

/// Crée un nouveau fichier CSV et y écrit des données
pub fn write_csv(filename: &str, people: Vec<Person>) -> Result<(), csv::Error> {
    println!("\n=== Écriture dans le fichier: {} ===", filename);
    
    let file = std::fs::File::create(filename)?;
    let mut wtr = WriterBuilder::new().from_writer(file);
    
    // Ajouter manuellement les en-têtes
    wtr.write_record(&["name", "age", "city"])?;
    
    // Écrire chaque enregistrement dans le fichier CSV
    for person in &people  {
        wtr.write_record(&[
            person.name.clone(),
            person.age.to_string(),
            person.city.clone(),
        ])?;
    }
    
    wtr.flush()?;
    let count = people.len();

    println!("✓ {} enregistrements écrits avec succès", count);
    
    Ok(())
}

/// Écrit des données en utilisant la sérialisation automatique
pub fn write_csv_serialize(filename: &str, people: &[Person]) -> Result<(), csv::Error> {
    println!("\n=== Écriture avec sérialisation dans: {} ===", filename);
    
    let file = std::fs::File::create(filename)?;
    let mut wtr = WriterBuilder::new().from_writer(file);
    
    // Écrire chaque enregistrement avec sérialisation automatique
    for person in people {
        wtr.serialize(person)?;
    }
    
    wtr.flush()?;
    println!("✓ {} enregistrements écrits avec succès", people.len());
    
    Ok(())
}

/// Retourne des données de test
pub fn get_sample_data() -> Vec<Person> {
    vec![
        Person::new("John".to_string(), 30, "New York".to_string()),
        Person::new("Jane".to_string(), 25, "San Francisco".to_string()),
        Person::new("Bob".to_string(), 35, "Chicago".to_string()),
    ]
}
