use csv::ReaderBuilder;
use crate::models::Person;

/// Lit un fichier CSV et affiche son contenu ligne par ligne (brut)
pub fn read_csv_raw(filename: &str) -> Result<(), csv::Error> {
    println!("\n=== Lecture brute du fichier: {} ===", filename);
    
    let file = std::fs::File::open(filename)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    
    // Itérer sur chaque enregistrement et afficher les valeurs
    for result in rdr.records() {
        let record = result?;
        for field in record.iter() {
            print!("{} ", field);
        }
        println!();
    }
    
    Ok(())
}

/// Lit un fichier CSV et le désérialise en structures Person
pub fn read_csv_deserialize(filename: &str) -> Result<Vec<Person>, csv::Error> {
    println!("\n=== Lecture et désérialisation du fichier: {} ===", filename);
    
    let file = std::fs::File::open(filename)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    
    // Collecter les enregistrements dans un vecteur de structures Person
    let people: Vec<Person> = rdr
        .deserialize()
        .filter_map(|result| result.ok())
        .collect();
    
    println!("✓ {} personnes lues", people.len());
    
    Ok(people)
}
