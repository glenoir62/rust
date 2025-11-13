# 🚀 Quick Start - CSV Management

## ⚡ Démarrage en 30 secondes

```bash
# 1. Compiler
cargo build

# 2. Exécuter les exemples
cargo run

# 3. C'est tout ! 🎉
```

## 📖 Premier exemple simple

Créez un fichier `examples/simple.rs` :

```rust
use csv_management::*;

fn main() -> Result<(), csv::Error> {
    // 1. Créer des données
    let people = vec![
        Person::new("Alice".to_string(), 30, "Paris".to_string()),
        Person::new("Bob".to_string(), 25, "Lyon".to_string()),
    ];
    
    // 2. Écrire dans un fichier
    write_csv("my_data.csv", people)?;
    println!("✅ Fichier créé !");
    
    // 3. Lire le fichier
    let loaded = read_csv_deserialize("my_data.csv")?;
    println!("✅ {} personnes lues", loaded.len());
    
    // 4. Afficher
    for person in loaded {
        person.display();
    }
    
    Ok(())
}
```

Puis exécutez :
```bash
cargo run --example simple
```

## 🎯 Cas d'usage principaux

### 1️⃣ Lire un CSV

```rust
use csv_management::*;

let people = read_csv_deserialize("data.csv")?;
for person in people {
    println!("{}: {} ans", person.name, person.age);
}
```

### 2️⃣ Créer un CSV

```rust
use csv_management::*;

let people = vec![
    Person::new("John".to_string(), 30, "NYC".to_string()),
];
write_csv("output.csv", people)?;
```

### 3️⃣ Ajouter une ligne

```rust
use csv_management::*;

let new_person = Person::new("Jane".to_string(), 25, "LA".to_string());
append_to_csv("output.csv", &new_person)?;
```

### 4️⃣ Modifier une donnée

```rust
use csv_management::*;

update_age_by_name("output.csv", "John", 31)?;
```

### 5️⃣ Supprimer une ligne

```rust
use csv_management::*;

delete_by_name("output.csv", "Jane")?;
```

## 📂 Structure du projet

```
csv_management/
├── src/
│   ├── models.rs      # Structure Person
│   ├── reader.rs      # Lecture
│   ├── writer.rs      # Écriture
│   ├── appender.rs    # Ajout
│   ├── updater.rs     # Modification
│   ├── lib.rs         # API publique
│   └── main.rs        # Exemples
└── Cargo.toml         # Configuration
```

## 🔑 Concepts clés

### Le type `Person`

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub city: String,
}
```

### L'opérateur `?` pour les erreurs

```rust
// Au lieu de :
let file = match File::open("data.csv") {
    Ok(f) => f,
    Err(e) => return Err(e),
};

// Utilisez simplement :
let file = File::open("data.csv")?;
```

### Result<T, E>

Toutes les fonctions retournent `Result<T, csv::Error>` :
- `Ok(valeur)` si succès
- `Err(erreur)` si échec

## 💡 Exemples avancés

### Filtrer avant d'écrire

```rust
let people = read_csv_deserialize("data.csv")?;
let adults: Vec<Person> = people
    .into_iter()
    .filter(|p| p.age >= 18)
    .collect();
write_csv("adults.csv", adults)?;
```

### Modifier avec une closure personnalisée

```rust
update_csv("data.csv", "data.csv", |people| {
    for person in people.iter_mut() {
        if person.city == "Paris" {
            person.age += 1; // Joyeux anniversaire aux Parisiens !
        }
    }
})?;
```

### Statistiques sur les données

```rust
let people = read_csv_deserialize("data.csv")?;

let avg_age = people.iter()
    .map(|p| p.age as f64)
    .sum::<f64>() / people.len() as f64;

println!("Âge moyen : {:.1} ans", avg_age);
```

## ⚠️ Erreurs courantes

### ❌ Fichier non trouvé
```
Error: Os { code: 2, kind: NotFound, message: "..." }
```
**Solution** : Vérifiez le chemin du fichier

### ❌ Format CSV invalide
```
Error: UnequalLengths { ... }
```
**Solution** : Vérifiez que toutes les lignes ont le même nombre de colonnes

### ❌ Type incompatible
```
Error: Deserialize("invalid type: string ...")
```
**Solution** : Vérifiez que le CSV correspond à la structure Person

## 📚 Pour aller plus loin

1. **README.md** → Documentation complète
2. **ARCHITECTURE.md** → Comprendre l'organisation
3. **main.rs** → Exemples détaillés
4. **Documentation Rust** → https://doc.rust-lang.org/

## 🎓 Exercices pratiques

### Exercice 1 : Compteur de villes
Comptez combien de personnes vivent dans chaque ville.

### Exercice 2 : Export filtré
Créez une fonction qui exporte seulement les personnes d'une ville donnée.

### Exercice 3 : Validation
Ajoutez une validation pour rejeter les âges négatifs.

### Exercice 4 : CSV vers JSON
Créez une fonction pour exporter les données en JSON.

## 🆘 Besoin d'aide ?

- **Documentation** : Lisez les fichiers .md du projet
- **Exemples** : Regardez `src/main.rs`
- **Rust Book** : https://doc.rust-lang.org/book/
- **csv crate** : https://docs.rs/csv/

## ✨ Fonctionnalités disponibles

✅ Lecture CSV (brute ou structurée)  
✅ Écriture CSV (manuelle ou automatique)  
✅ Ajout d'enregistrements  
✅ Mise à jour par nom  
✅ Suppression par nom  
✅ Gestion d'erreurs avec `Result`  
✅ Sérialisation/Désérialisation automatique  

## 🎯 Bon à savoir

- Les fichiers CSV utilisent `,` comme séparateur
- Les en-têtes sont automatiquement gérés
- La structure `Person` est extensible
- Toutes les fonctions gèrent les erreurs proprement
- Le code est modulaire et réutilisable

---

**Prêt à coder ?** Lancez `cargo run` et explorez ! 🚀
