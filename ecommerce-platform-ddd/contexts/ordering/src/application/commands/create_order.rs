use crate::domain::{
    aggregates::Order,
    entities::OrderItem,
    errors::DomainError,
    repositories::OrderRepository,
    value_objects::{CustomerId, Money, OrderId, ProductId},
};
use crate::infrastructure::messaging::EventPublisher;
use rust_decimal::Decimal;
use std::sync::Arc;

/// Command: Create Order (CQRS Pattern)
#[derive(Debug)]
pub struct CreateOrderCommand {
    pub customer_id: CustomerId,
    pub items: Vec<CreateOrderItemDto>,
}

#[derive(Debug)]
pub struct CreateOrderItemDto {
    pub product_id: ProductId,
    pub product_name: String,
    pub quantity: u32,
    pub unit_price: Decimal,
}

/// Command Handler (Application Service)
/// Orchestrates the use case
pub struct CreateOrderHandler {
    order_repository: Arc<dyn OrderRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl CreateOrderHandler {
    pub fn new(
        order_repository: Arc<dyn OrderRepository>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            order_repository,
            event_publisher,
        }
    }

    /// Handle the command
    pub async fn handle(&self, command: CreateOrderCommand) -> Result<OrderId, DomainError> {
        // 1. Convert DTOs to domain entities
        let items: Vec<OrderItem> = command
            .items
            .into_iter()
            .map(|dto| {
                OrderItem::new(
                    dto.product_id,
                    dto.product_name,
                    dto.quantity,
                    Money::eur(dto.unit_price)?,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        // 2. Create aggregate (business logic in domain)
        let mut order = Order::create(command.customer_id, items)?;

        // 3. Persist
        self.order_repository.save(&mut order).await?;

        // 4. Publish domain events
        let events = order.take_events();
        for event in events {
            self.event_publisher.publish(event).await?;
        }

        Ok(order.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::persistence::repositories::InMemoryOrderRepository;
    use crate::infrastructure::messaging::NoOpEventPublisher;

    #[tokio::test]
    async fn test_create_order_command() {
        let repo = Arc::new(InMemoryOrderRepository::new());
        let publisher = Arc::new(NoOpEventPublisher);
        let handler = CreateOrderHandler::new(repo, publisher);

        let command = CreateOrderCommand {
            customer_id: CustomerId::new(),
            items: vec![CreateOrderItemDto {
                product_id: ProductId::new(),
                product_name: "Test Product".to_string(),
                quantity: 2,
                unit_price: Decimal::new(1000, 2),
            }],
        };

        let result = handler.handle(command).await;
        assert!(result.is_ok());
    }
}
