# CSV Management - Projet Rust

Un projet Rust complet et bien structuré pour la gestion de fichiers CSV avec différentes opérations CRUD.

## 📁 Structure du projet

```
csv_management/
├── Cargo.toml              # Configuration et dépendances
├── README.md               # Documentation
└── src/
    ├── lib.rs              # Point d'entrée de la bibliothèque
    ├── main.rs             # Exemples d'utilisation
    ├── models.rs           # Structures de données (Person)
    ├── reader.rs           # Lecture de fichiers CSV
    ├── writer.rs           # Écriture de fichiers CSV
    ├── appender.rs         # Ajout d'enregistrements
    └── updater.rs          # Mise à jour et suppression
```

## 🎯 Fonctionnalités

### 1. **models.rs** - Structures de données
- Structure `Person` avec sérialisation/désérialisation
- Méthodes utilitaires (new, display)

### 2. **reader.rs** - Lecture CSV
- `read_csv_raw()` : Lecture brute ligne par ligne
- `read_csv_deserialize()` : Désérialisation en structures Person

### 3. **writer.rs** - Écriture CSV
- `write_csv()` : Écriture manuelle avec en-têtes
- `write_csv_serialize()` : Écriture automatique avec sérialisation
- `get_sample_data()` : Données de test

### 4. **appender.rs** - Ajout d'enregistrements
- `append_to_csv()` : Ajoute un enregistrement à un fichier existant
- Crée le fichier avec en-têtes s'il n'existe pas

### 5. **updater.rs** - Mise à jour et suppression
- `update_csv()` : Fonction générique pour modifier des données
- `update_age_by_name()` : Met à jour l'âge d'une personne
- `delete_by_name()` : Supprime une personne par son nom

## 🚀 Utilisation

### Compiler le projet
```bash
cargo build
```

### Exécuter les exemples
```bash
cargo run
```

### Utiliser comme bibliothèque

```rust
use csv_management::*;

fn main() -> Result<(), csv::Error> {
    // Créer des données
    let people = vec![
        Person::new("John".to_string(), 30, "Paris".to_string()),
    ];
    
    // Écrire dans un fichier
    write_csv("data.csv", people)?;
    
    // Lire le fichier
    let loaded = read_csv_deserialize("data.csv")?;
    
    // Ajouter un enregistrement
    let new_person = Person::new("Jane".to_string(), 25, "Lyon".to_string());
    append_to_csv("data.csv", &new_person)?;
    
    // Mettre à jour
    update_age_by_name("data.csv", "John", 31)?;
    
    // Supprimer
    delete_by_name("data.csv", "Jane")?;
    
    Ok(())
}
```

## 📦 Dépendances

- `csv = "1.3"` : Manipulation de fichiers CSV
- `serde = { version = "1.0", features = ["derive"] }` : Sérialisation/Désérialisation

## 🎨 Principes d'organisation

1. **Séparation des responsabilités** : Chaque module a une responsabilité claire
2. **Réutilisabilité** : Le modèle `Person` est centralisé dans `models.rs`
3. **Modularité** : Chaque opération (lire, écrire, modifier) est dans son propre module
4. **Facilité d'utilisation** : `lib.rs` ré-exporte tout pour un usage simple
5. **Exemples complets** : `main.rs` montre toutes les fonctionnalités

## 💡 Cas d'usage couverts

✅ Lecture brute de CSV  
✅ Lecture avec désérialisation automatique  
✅ Écriture manuelle de CSV  
✅ Écriture avec sérialisation automatique  
✅ Ajout d'enregistrements (append)  
✅ Mise à jour d'enregistrements  
✅ Suppression d'enregistrements  

## 📝 Notes

- Tous les fichiers CSV sont créés dans le répertoire courant
- La gestion d'erreurs utilise le type `Result<T, csv::Error>`
- L'opérateur `?` propage automatiquement les erreurs
