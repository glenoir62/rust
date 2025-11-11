# E-Commerce Platform - DDD with Axum + SeaORM + Iggy

## 🏗️ Architecture

Ce projet est une démonstration complète d'architecture **Domain-Driven Design (DDD)** en Rust avec:
- **Axum** pour le framework web
- **SeaORM** pour l'ORM
- **Iggy** pour le message broker (event-driven architecture)

## 📐 Structure DDD

```
contexts/
├── ordering/          # Bounded Context: Order Management
│   ├── domain/        # Couche Domain (Pure Business Logic)
│   │   ├── aggregates/      # Aggregate Roots (Order)
│   │   ├── entities/        # Entities (OrderItem)
│   │   ├── value_objects/   # Value Objects (Money, OrderStatus, IDs)
│   │   ├── events/          # Domain Events
│   │   ├── repositories/    # Repository Traits (Ports)
│   │   └── services/        # Domain Services
│   ├── application/   # Couche Application (Use Cases)
│   │   ├── commands/        # Command Handlers (CQRS)
│   │   ├── queries/         # Query Handlers (CQRS)
│   │   └── dto/             # Data Transfer Objects
│   ├── infrastructure/# Couche Infrastructure (Adapters)
│   │   ├── persistence/     # Database (SeaORM)
│   │   ├── messaging/       # Event Bus (Iggy)
│   │   └── api/             # REST API (Axum)
│   └── presentation/  # Couche Présentation
│       └── main.rs          # Application entry point
```

## 🚀 Démarrage rapide

### Prérequis
- Rust 1.75+
- PostgreSQL 16+
- Docker & Docker Compose (optionnel)

### Installation

```bash
# Cloner le projet
git clone <repository-url>
cd ecommerce-platform-ddd

# Démarrer les services (PostgreSQL + Iggy)
docker-compose up -d

# Créer la base de données
createdb orders

# Lancer les migrations
cd contexts/ordering
cargo run --bin migrate

# Démarrer l'API
cargo run
```

L'API sera disponible sur `http://localhost:3000`

## 📝 Endpoints API

### Orders

```bash
# Créer une commande
POST /api/orders
Content-Type: application/json

{
  "customer_id": "uuid",
  "items": [
    {
      "product_id": "uuid",
      "product_name": "Product A",
      "quantity": 2,
      "unit_price": "10.00"
    }
  ]
}

# Récupérer une commande
GET /api/orders/{order_id}

# Lister les commandes d'un client
GET /api/customers/{customer_id}/orders

# Confirmer une commande
POST /api/orders/{order_id}/confirm

# Annuler une commande
POST /api/orders/{order_id}/cancel
{
  "reason": "Customer request"
}
```

## 🎯 Concepts DDD implémentés

### ✅ Tactical Patterns

- **Aggregates** : `Order` (aggregate root) avec `OrderItem` (entities)
- **Value Objects** : `Money`, `OrderStatus`, typed IDs (`OrderId`, `CustomerId`, etc.)
- **Domain Events** : `OrderCreated`, `OrderPaid`, `OrderShipped`, etc.
- **Repositories** : Interface (trait) dans le domain, implémentation dans l'infrastructure
- **Domain Services** : Logique métier qui ne rentre pas dans un aggregate

### ✅ Strategic Patterns

- **Bounded Contexts** : `ordering`, `payment`, `notification` (séparés)
- **Ubiquitous Language** : Terminologie métier partout (Order, Money, not Record/Amount)
- **Event-Driven Architecture** : Communication inter-contexts via Iggy

### ✅ Architecture Patterns

- **Hexagonal Architecture** (Ports & Adapters)
- **CQRS** (Command Query Responsibility Segregation)
- **Clean Architecture** (Dependency Rule: domain → application → infrastructure)

## 🧪 Tests

```bash
# Tests unitaires (domain layer)
cargo test --lib

# Tests d'intégration
cargo test --test integration

# Tous les tests
cargo test
```

## 🔧 Technologies

| Layer | Technology |
|-------|------------|
| Web Framework | Axum 0.7 |
| ORM | SeaORM 0.12 |
| Message Broker | Iggy 0.6 |
| Database | PostgreSQL 16 |
| Async Runtime | Tokio |
| Serialization | Serde |
| Error Handling | thiserror, anyhow |
| Logging | tracing |

## 📚 Ressources d'apprentissage

### DDD
- [Domain-Driven Design (Eric Evans)](https://www.domainlanguage.com/ddd/)
- [Implementing Domain-Driven Design (Vaughn Vernon)](https://vaughnvernon.com/)

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Async Rust](https://rust-lang.github.io/async-book/)

### Axum
- [Axum Documentation](https://docs.rs/axum)

### SeaORM
- [SeaORM Documentation](https://www.sea-ql.org/SeaORM/)

### Iggy
- [Iggy Documentation](https://github.com/iggy-rs/iggy)

## 🏛️ Principes respectés

- **Separation of Concerns** : Chaque couche a une responsabilité claire
- **Dependency Inversion** : Le domain ne dépend de rien
- **Single Responsibility** : Chaque aggregate gère ses propres invariants
- **Tell, Don't Ask** : Les aggregates encapsulent leur état
- **Immutability** : Value objects sont immutables

## 🎓 Pour les développeurs Java/Spring Boot

Ce projet est l'équivalent Rust de :
- **Spring Boot** → Axum
- **Hibernate/JPA** → SeaORM
- **Kafka/RabbitMQ** → Iggy
- **@Entity** → SeaORM models
- **@Aggregate** → Order aggregate
- **@Service** → Command/Query handlers
- **@Repository** → OrderRepository trait + implémentation

## 📊 Comparaison Java vs Rust

| Java/Spring | Ce projet Rust |
|-------------|----------------|
| `@RestController` | `async fn handlers` |
| `@Autowired` | `State<Arc<Handler>>` |
| `@Entity` | `DeriveEntityModel` |
| `Optional<T>` | `Option<T>` |
| `throws Exception` | `Result<T, E>` |
| `null` | N'existe pas! |
| Runtime reflection | Compile-time macros |

## 🤝 Contribution

Les contributions sont bienvenues ! Voir [CONTRIBUTING.md](CONTRIBUTING.md)

## 📄 Licence

MIT License - voir [LICENSE](LICENSE)

## 👨‍💻 Auteur

**GLEO** - Expert Java/Liferay & Rust Architecture

---

**Note** : Ce projet est un template éducatif pour apprendre DDD en Rust. Il contient des implémentations complètes mais simplifiées pour la clarté pédagogique.
