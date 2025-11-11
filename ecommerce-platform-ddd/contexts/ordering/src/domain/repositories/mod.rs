use crate::domain::{
    aggregates::Order,
    errors::DomainError,
    value_objects::{CustomerId, OrderId},
};
use async_trait::async_trait;

/// Repository trait (Port in Hexagonal Architecture)
/// The domain defines what it needs, infrastructure implements how
#[async_trait]
pub trait OrderRepository: Send + Sync {
    /// Save or update an order
    async fn save(&self, order: &mut Order) -> Result<(), DomainError>;

    /// Find order by ID
    async fn find_by_id(&self, id: OrderId) -> Result<Option<Order>, DomainError>;

    /// Find all orders for a customer
    async fn find_by_customer(&self, customer_id: CustomerId) -> Result<Vec<Order>, DomainError>;

    /// Delete an order
    async fn delete(&self, id: OrderId) -> Result<(), DomainError>;

    /// Get next order ID (for event sourcing scenarios)
    fn next_id(&self) -> OrderId {
        OrderId::new()
    }
}
