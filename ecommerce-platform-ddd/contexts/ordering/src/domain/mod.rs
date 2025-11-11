pub mod aggregates;
pub mod entities;
pub mod errors;
pub mod events;
pub mod repositories;
pub mod value_objects;

// Re-exports for convenience
pub use aggregates::Order;
pub use entities::OrderItem;
pub use errors::DomainError;
pub use events::OrderEvent;
pub use repositories::OrderRepository;
pub use value_objects::{CustomerId, Money, OrderId, OrderItemId, OrderStatus, ProductId};
