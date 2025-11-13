mod writer;

use csv_processor::*;

fn main() -> Result<(), csv::Error> {
    println!("╔═════════════════════════════════════════════════════════════╗");
    println!("║        Démonstration de gestion de fichiers CSV            ║");
    println!("╚═════════════════════════════════════════════════════════════╝");

    // 1. Écrire un fichier CSV initial
    demo_write_csv()?;
    
    // 2. Lire le fichier CSV (brut)
    demo_read_csv_raw()?;
    
    // 3. Lire et désérialiser le fichier CSV
    demo_read_csv_deserialize()?;
    
    // 4. Ajouter un enregistrement
    demo_append_csv()?;
    
    // 5. Mettre à jour un enregistrement
    demo_update_csv()?;
    
    // 6. Écrire avec sérialisation
    demo_write_serialize()?;
    
    // 7. Supprimer un enregistrement
    demo_delete_from_csv()?;

    println!("\n✅ Toutes les opérations ont été effectuées avec succès!");
    
    Ok(())
}

/// Démo 1: Écrire un fichier CSV initial
fn demo_write_csv() -> Result<(), csv::Error> {
    println!("\n┌─────────────────────────────────────────────────────────┐");
    println!("│ DÉMO 1: Écriture d'un fichier CSV                      │");
    println!("└─────────────────────────────────────────────────────────┘");
    
    let people = get_sample_data();
    write_csv("output.csv", people)?;
    
    Ok(())
}

/// Démo 2: Lecture brute d'un fichier CSV
fn demo_read_csv_raw() -> Result<(), csv::Error> {
    println!("\n┌─────────────────────────────────────────────────────────┐");
    println!("│ DÉMO 2: Lecture brute du fichier CSV                   │");
    println!("└─────────────────────────────────────────────────────────┘");
    
    read_csv_raw("output.csv")?;
    
    Ok(())
}

/// Démo 3: Lecture avec désérialisation
fn demo_read_csv_deserialize() -> Result<(), csv::Error> {
    println!("\n┌─────────────────────────────────────────────────────────┐");
    println!("│ DÉMO 3: Lecture avec désérialisation                   │");
    println!("└─────────────────────────────────────────────────────────┘");
    
    let people = read_csv_deserialize("output.csv")?;
    
    println!("\nPersonnes trouvées:");
    for person in people {
        person.display();
    }
    
    Ok(())
}

/// Démo 4: Ajouter un enregistrement
fn demo_append_csv() -> Result<(), csv::Error> {
    println!("\n┌─────────────────────────────────────────────────────────┐");
    println!("│ DÉMO 4: Ajout d'un enregistrement                      │");
    println!("└─────────────────────────────────────────────────────────┘");
    
    let new_person = Person::new("Alice".to_string(), 28, "Seattle".to_string());
    append_to_csv("output.csv", &new_person)?;
    
    // Relire pour vérifier
    let people = read_csv_deserialize("output.csv")?;
    println!("\nNombre total de personnes: {}", people.len());
    
    Ok(())
}

/// Démo 5: Mettre à jour un enregistrement
fn demo_update_csv() -> Result<(), csv::Error> {
    println!("\n┌─────────────────────────────────────────────────────────┐");
    println!("│ DÉMO 5: Mise à jour d'un enregistrement                │");
    println!("└─────────────────────────────────────────────────────────┘");
    
    update_age_by_name("output.csv", "Jane", 26)?;
    
    // Relire pour vérifier
    let people = read_csv_deserialize("output.csv")?;
    if let Some(jane) = people.iter().find(|p| p.name == "Jane") {
        println!("\nVérification:");
        jane.display();
    }
    
    Ok(())
}

/// Démo 6: Écrire avec sérialisation automatique
fn demo_write_serialize() -> Result<(), csv::Error> {
    println!("\n┌─────────────────────────────────────────────────────────┐");
    println!("│ DÉMO 6: Écriture avec sérialisation automatique        │");
    println!("└─────────────────────────────────────────────────────────┘");
    
    let people = vec![
        Person::new("Charlie".to_string(), 40, "Boston".to_string()),
        Person::new("Diana".to_string(), 33, "Austin".to_string()),
    ];
    
    write_csv_serialize("serialized.csv", &people)?;
    
    Ok(())
}

/// Démo 7: Supprimer un enregistrement
fn demo_delete_from_csv() -> Result<(), csv::Error> {
    println!("\n┌─────────────────────────────────────────────────────────┐");
    println!("│ DÉMO 7: Suppression d'un enregistrement                │");
    println!("└─────────────────────────────────────────────────────────┘");
    
    delete_by_name("output.csv", "Bob")?;
    
    // Relire pour vérifier
    let people = read_csv_deserialize("output.csv")?;
    println!("\nPersonnes restantes:");
    for person in people {
        person.display();
    }
    
    Ok(())
}
