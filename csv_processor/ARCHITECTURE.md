# Architecture du projet CSV Management

## 🏗️ Vue d'ensemble

```
┌─────────────────────────────────────────────────────────────┐
│                        main.rs                              │
│                   (Point d'entrée)                          │
│           Démontre toutes les fonctionnalités               │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   │ utilise
                   ↓
┌─────────────────────────────────────────────────────────────┐
│                        lib.rs                               │
│              (Interface publique)                           │
│     Ré-exporte tous les modules et fonctions                │
└──────────────────┬──────────────────────────────────────────┘
                   │
        ┌──────────┼──────────┬──────────┬──────────┐
        │          │          │          │          │
        ↓          ↓          ↓          ↓          ↓
    ┌───────┐ ┌────────┐ ┌────────┐ ┌─────────┐ ┌─────────┐
    │models │ │reader  │ │writer  │ │appender │ │updater  │
    │  .rs  │ │  .rs   │ │  .rs   │ │  .rs    │ │  .rs    │
    └───────┘ └────────┘ └────────┘ └─────────┘ └─────────┘
```

## 📋 Détail des modules

### 1. models.rs - Le cœur des données
```
┌─────────────────────────────────────┐
│         struct Person               │
├─────────────────────────────────────┤
│ + name: String                      │
│ + age: u32                          │
│ + city: String                      │
├─────────────────────────────────────┤
│ + new() → Person                    │
│ + display()                         │
└─────────────────────────────────────┘
    ↑
    │ utilisé par tous les modules
    │
```

### 2. reader.rs - Lecture des données
```
┌──────────────────────────────────────┐
│        Module Reader                 │
├──────────────────────────────────────┤
│ read_csv_raw()                       │
│  → Lecture brute (String)            │
│                                      │
│ read_csv_deserialize()               │
│  → Lecture structurée (Vec<Person>)  │
└──────────────────────────────────────┘
```

### 3. writer.rs - Écriture des données
```
┌──────────────────────────────────────┐
│        Module Writer                 │
├──────────────────────────────────────┤
│ write_csv()                          │
│  → Écriture manuelle                 │
│                                      │
│ write_csv_serialize()                │
│  → Écriture automatique (serde)      │
│                                      │
│ get_sample_data()                    │
│  → Données de test                   │
└──────────────────────────────────────┘
```

### 4. appender.rs - Ajout de données
```
┌──────────────────────────────────────┐
│        Module Appender               │
├──────────────────────────────────────┤
│ append_to_csv()                      │
│  → Ajoute sans écraser               │
│  → Crée le fichier si nécessaire     │
└──────────────────────────────────────┘
```

### 5. updater.rs - Modification des données
```
┌──────────────────────────────────────┐
│        Module Updater                │
├──────────────────────────────────────┤
│ update_csv()                         │
│  → Fonction générique de modification│
│                                      │
│ update_age_by_name()                 │
│  → Mise à jour spécifique            │
│                                      │
│ delete_by_name()                     │
│  → Suppression par nom               │
└──────────────────────────────────────┘
```

## 🔄 Flux de données typiques

### Flux 1: Création d'un fichier CSV
```
main.rs
  → get_sample_data() [writer.rs]
  → write_csv() [writer.rs]
  → Fichier CSV créé ✓
```

### Flux 2: Lecture et affichage
```
main.rs
  → read_csv_deserialize() [reader.rs]
  → Vec<Person> [models.rs]
  → person.display() [models.rs]
  → Affichage console ✓
```

### Flux 3: Modification de données
```
main.rs
  → update_age_by_name() [updater.rs]
    → read_csv_deserialize() [reader.rs]
    → Modification Vec<Person>
    → write_csv_serialize() [writer.rs]
  → Fichier mis à jour ✓
```

### Flux 4: Ajout d'un enregistrement
```
main.rs
  → Person::new() [models.rs]
  → append_to_csv() [appender.rs]
  → Enregistrement ajouté ✓
```

## 🎯 Principes de conception

### Séparation des responsabilités (SRP)
- **models.rs** : Définition des données uniquement
- **reader.rs** : Lecture uniquement
- **writer.rs** : Écriture uniquement
- **appender.rs** : Ajout uniquement
- **updater.rs** : Modification/Suppression uniquement

### DRY (Don't Repeat Yourself)
- Structure `Person` définie une seule fois dans `models.rs`
- Tous les modules l'importent via `use crate::models::Person`

### Interface claire
- `lib.rs` expose une API simple et cohérente
- Utilisateurs n'ont pas besoin de connaître l'organisation interne

### Extensibilité
Facile d'ajouter de nouvelles fonctionnalités :
- Nouveau module → Ajout dans `lib.rs`
- Nouvelle opération → Ajout dans le module approprié
- Nouveau type de données → Ajout dans `models.rs`

## 📊 Comparaison : Avant / Après refactoring

### ❌ Avant (Code monolithique)
```
main.rs (500 lignes)
├── struct Person { ... }
├── fn read_csv() { ... }
├── fn write_csv() { ... }
├── fn append_csv() { ... }
├── fn update_csv() { ... }
└── fn main() { ... }
```
**Problèmes** :
- Difficile à maintenir
- Code dupliqué
- Tests difficiles
- Pas réutilisable

### ✅ Après (Architecture modulaire)
```
csv_management/
├── models.rs      (50 lignes)
├── reader.rs      (40 lignes)
├── writer.rs      (60 lignes)
├── appender.rs    (40 lignes)
├── updater.rs     (70 lignes)
├── lib.rs         (15 lignes)
└── main.rs        (100 lignes)
```
**Avantages** :
- ✅ Code organisé et maintenable
- ✅ Chaque module a une responsabilité claire
- ✅ Facile à tester individuellement
- ✅ Réutilisable comme bibliothèque

## 🧪 Testabilité

Chaque module peut être testé indépendamment :

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_person_creation() {
        let person = Person::new("Test".to_string(), 25, "Paris".to_string());
        assert_eq!(person.name, "Test");
    }
}
```

## 🚀 Évolutions futures possibles

1. **Ajout de filtres** : Module `filter.rs` pour filtrer les données
2. **Support de formats multiples** : Module `formats/` (JSON, XML)
3. **Base de données** : Module `database.rs` pour persister en DB
4. **API REST** : Module `api.rs` avec Axum
5. **Tests unitaires** : Module `tests/` complet
6. **Benchmarks** : Module `benches/` pour performances

## 📚 Ressources

- [The Rust Book - Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [csv crate documentation](https://docs.rs/csv/)
