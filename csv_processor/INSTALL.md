# Installation et Configuration

## 📋 Prérequis

1. **Rust** installé sur votre machine
   - Vérifier : `rustc --version`
   - Si non installé : https://rustup.rs/

2. **Cargo** (installé automatiquement avec Rust)
   - Vérifier : `cargo --version`

## 🚀 Installation

### Option 1 : Cloner depuis ce projet

```bash
# Naviguer vers le répertoire du projet
cd csv_management

# Compiler le projet
cargo build

# Exécuter les exemples
cargo run

# Exécuter en mode release (optimisé)
cargo run --release
```

### Option 2 : Créer de zéro

```bash
# Créer un nouveau projet
cargo new csv_management
cd csv_management

# Ajouter les dépendances dans Cargo.toml
# (voir le fichier Cargo.toml du projet)

# Copier les fichiers src/*
# (voir tous les fichiers du dossier src/)
```

## 📦 Structure des fichiers à copier

```
csv_management/
├── Cargo.toml          ← Configuration du projet
├── README.md           ← Documentation générale
├── ARCHITECTURE.md     ← Documentation de l'architecture
├── INSTALL.md          ← Ce fichier
├── data.csv            ← Fichier de test
└── src/
    ├── lib.rs          ← Point d'entrée bibliothèque
    ├── main.rs         ← Point d'entrée exécutable
    ├── models.rs       ← Structures de données
    ├── reader.rs       ← Lecture CSV
    ├── writer.rs       ← Écriture CSV
    ├── appender.rs     ← Ajout d'enregistrements
    └── updater.rs      ← Mise à jour/Suppression
```

## 🧪 Tester le projet

### Exécuter tous les exemples
```bash
cargo run
```

### Compiler sans exécuter
```bash
cargo build
```

### Vérifier la syntaxe
```bash
cargo check
```

### Formater le code
```bash
cargo fmt
```

### Analyser le code (linter)
```bash
cargo clippy
```

## 📝 Commandes utiles

### Créer un nouveau binaire
```bash
# Dans Cargo.toml, ajouter :
[[bin]]
name = "mon_app"
path = "src/bin/mon_app.rs"

# Puis exécuter :
cargo run --bin mon_app
```

### Utiliser comme bibliothèque dans un autre projet

Dans le `Cargo.toml` de votre autre projet :

```toml
[dependencies]
csv_management = { path = "../csv_management" }
```

Puis dans votre code :

```rust
use csv_management::*;

fn main() -> Result<(), csv::Error> {
    let people = get_sample_data();
    write_csv("output.csv", people)?;
    Ok(())
}
```

## 🔧 Dépendances

Le projet utilise :

```toml
[dependencies]
csv = "1.3"                              # Manipulation CSV
serde = { version = "1.0", features = ["derive"] }  # Sérialisation
```

Pour mettre à jour les dépendances :
```bash
cargo update
```

## 🐛 Résolution de problèmes

### Erreur : "csv crate not found"
```bash
cargo clean
cargo build
```

### Erreur de compilation avec serde
Vérifier que `features = ["derive"]` est bien présent dans Cargo.toml

### Fichier CSV non trouvé
Les fichiers CSV sont cherchés dans le répertoire courant.
Exécuter depuis la racine du projet : `cargo run`

## 📚 Ressources supplémentaires

- [Documentation Rust](https://doc.rust-lang.org/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [csv crate](https://docs.rs/csv/)
- [serde documentation](https://serde.rs/)

## 💡 Conseils

1. **Toujours compiler avant de commit** : `cargo build`
2. **Utiliser clippy régulièrement** : `cargo clippy`
3. **Formater le code** : `cargo fmt`
4. **Tester régulièrement** : `cargo test` (quand vous ajoutez des tests)

## 🎯 Prochaines étapes

Après installation :

1. Lire `README.md` pour comprendre les fonctionnalités
2. Lire `ARCHITECTURE.md` pour comprendre l'organisation
3. Exécuter `cargo run` pour voir les exemples
4. Modifier `main.rs` pour vos propres besoins
5. Créer vos propres modules selon vos besoins
