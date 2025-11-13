use csv::WriterBuilder;
use std::fs::OpenOptions;
use crate::models::Person;

/// Ajoute une nouvelle personne à un fichier CSV existant (ou le crée s'il n'existe pas)
pub fn append_to_csv(filename: &str, person: &Person) -> Result<(), csv::Error> {
    println!("\n=== Ajout d'un enregistrement dans: {} ===", filename);
    
    let file_exists = std::path::Path::new(filename).exists();
    
    // Ouvrir le fichier CSV en mode ajout ou créer s'il n'existe pas
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)?;
    
    let mut wtr = WriterBuilder::new().from_writer(file);
    
    if !file_exists {
        // Ajouter manuellement les en-têtes si le fichier est nouveau
        wtr.write_record(&["name", "age", "city"])?;
        println!("✓ Nouveau fichier créé avec en-têtes");
    }
    
    wtr.write_record(&[
        person.name.clone(),
        person.age.to_string(),
        person.city.clone(),
    ])?;
    
    wtr.flush()?;
    println!("✓ Enregistrement ajouté: {} ({} ans, {})", person.name, person.age, person.city);
    
    Ok(())
}
