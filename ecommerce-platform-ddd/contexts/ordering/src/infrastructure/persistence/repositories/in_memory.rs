use crate::domain::{
    aggregates::Order,
    errors::DomainError,
    repositories::OrderRepository,
    value_objects::{CustomerId, OrderId},
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// In-memory implementation for testing
pub struct InMemoryOrderRepository {
    orders: Arc<RwLock<HashMap<OrderId, Order>>>,
}

impl InMemoryOrderRepository {
    pub fn new() -> Self {
        Self {
            orders: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl OrderRepository for InMemoryOrderRepository {
    async fn save(&self, order: &mut Order) -> Result<(), DomainError> {
        let mut orders = self.orders.write().await;
        orders.insert(order.id(), order.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: OrderId) -> Result<Option<Order>, DomainError> {
        let orders = self.orders.read().await;
        Ok(orders.get(&id).cloned())
    }

    async fn find_by_customer(&self, customer_id: CustomerId) -> Result<Vec<Order>, DomainError> {
        let orders = self.orders.read().await;
        Ok(orders
            .values()
            .filter(|o| o.customer_id() == customer_id)
            .cloned()
            .collect())
    }

    async fn delete(&self, id: OrderId) -> Result<(), DomainError> {
        let mut orders = self.orders.write().await;
        orders.remove(&id);
        Ok(())
    }
}

// Implement Clone for Order (needed for in-memory storage)
// This would go in the Order implementation
