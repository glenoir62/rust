# 🚀 Guide de Démarrage Rapide

## Pour GLEO - Expert Java/Spring Boot

Bienvenue dans ton projet d'apprentissage Rust avec architecture DDD !

## 📋 Checklist d'installation

### 1. Installer Rust (si pas déjà fait)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Vérifier l'installation

```bash
rustc --version
cargo --version
```

### 3. Cloner et setup

```bash
cd ecommerce-platform-ddd
cargo check  # Vérifie que tout compile
```

## 🎯 Progression d'apprentissage recommandée

### Semaine 1-2 : Comprendre la structure DDD

1. **Lire le README.md** principal
2. **Explorer la structure** :
   ```bash
   tree contexts/ordering/src/domain -L 2
   ```
3. **Lancer les tests** :
   ```bash
   cd contexts/ordering
   cargo test
   ```

### Semaine 3-4 : Modifier et étendre

1. Ajouter un nouveau Value Object (ex: `Address`)
2. Ajouter un nouveau Command (ex: `UpdateOrderCommand`)
3. Implémenter un Query Handler

### Semaine 5-6 : Infrastructure

1. Implémenter le vrai repository SeaORM
2. Connecter Iggy pour les events
3. Ajouter des endpoints REST

## 📚 Fichiers à lire dans l'ordre

1. `contexts/ordering/src/domain/value_objects/money.rs` - Value Objects
2. `contexts/ordering/src/domain/entities/order_item.rs` - Entities
3. `contexts/ordering/src/domain/aggregates/order.rs` - Aggregate Root
4. `contexts/ordering/src/domain/events/mod.rs` - Domain Events
5. `contexts/ordering/src/application/commands/create_order.rs` - Use Case

## 🔧 Commandes utiles

```bash
# Compiler
cargo build

# Compiler en release (optimisé)
cargo build --release

# Lancer les tests
cargo test

# Lancer les tests avec logs
cargo test -- --nocapture

# Formater le code
cargo fmt

# Linter
cargo clippy

# Vérifier sans compiler
cargo check

# Voir la documentation
cargo doc --open
```

## 💡 Équivalences Spring Boot

| Ce que tu cherches | Où c'est dans ce projet |
|--------------------|------------------------|
| `@Entity` | `contexts/ordering/src/domain/aggregates/order.rs` |
| `@Service` | `contexts/ordering/src/application/commands/` |
| `@Repository` | `contexts/ordering/src/domain/repositories/mod.rs` (trait) |
| Repository impl | `contexts/ordering/src/infrastructure/persistence/` |
| `@RestController` | `contexts/ordering/src/infrastructure/api/rest/` (à implémenter) |
| `application.properties` | `.env` ou variables d'environnement |

## 🐛 Debugging

### Erreur de compilation ?

Les erreurs Rust sont très explicites. Lis-les attentivement, elles te guident !

```bash
# Pour avoir plus de détails
cargo build --verbose
```

### Besoin d'aide ?

1. Lis le message d'erreur du compilateur (il est ton ami !)
2. Cherche dans la doc : https://doc.rust-lang.org/book/
3. Demande dans le Discord Rust francophone

## 📖 Ressources

- [The Rust Book](https://doc.rust-lang.org/book/) - LA référence
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples)
- [SeaORM Tutorial](https://www.sea-ql.org/SeaORM/docs/index/)

## 🎓 Exercices proposés

### Exercice 1 : Nouveau Value Object
Créer un `EmailAddress` value object avec validation

### Exercice 2 : Nouveau Command
Implémenter `ConfirmOrderCommand` et son handler

### Exercice 3 : Query Side
Implémenter `GetOrderQuery` pour lire une commande

### Exercice 4 : REST API
Créer les endpoints REST dans `infrastructure/api/rest/`

## ✨ Bon courage !

N'oublie pas : Rust est difficile au début, mais une fois que tu maîtrises le borrow checker, tu ne voudras plus revenir en arrière !

Le compilateur Rust est strict, mais c'est ton meilleur ami - il détecte les bugs avant l'exécution.

**Happy Coding! 🦀**
