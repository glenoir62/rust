use crate::domain::{
    entities::OrderItem,
    events::OrderEvent,
    value_objects::{CustomerId, Money, OrderId, OrderStatus},
    errors::DomainError,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

/// Order Aggregate Root
/// Enforces invariants and business rules
/// All modifications must go through this aggregate
#[derive(Debug, Clone)]
pub struct Order {
    // Identity
    id: OrderId,
    customer_id: CustomerId,

    // State
    items: Vec<OrderItem>,
    status: OrderStatus,
    total: Money,

    // Audit
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    // Domain Events (not persisted, collected for publishing)
    domain_events: Vec<OrderEvent>,
}

impl Order {
    /// Factory method - only way to create a valid Order
    pub fn create(customer_id: CustomerId, items: Vec<OrderItem>) -> Result<Self, DomainError> {
        // Business rule: order must have at least one item
        if items.is_empty() {
            return Err(DomainError::EmptyOrder);
        }

        // Calculate total (business logic in aggregate)
        let total = Self::calculate_total(&items)?;

        let order_id = OrderId::new();
        let now = Utc::now();

        let mut order = Self {
            id: order_id,
            customer_id,
            items,
            status: OrderStatus::Pending,
            total,
            created_at: now,
            updated_at: now,
            domain_events: Vec::new(),
        };

        // Raise domain event
        order.add_event(OrderEvent::OrderCreated {
            order_id: order.id,
            customer_id: order.customer_id,
            total: order.total,
            timestamp: now,
        });

        Ok(order)
    }

    /// Business logic: confirm the order
    pub fn confirm(&mut self) -> Result<(), DomainError> {
        if !self.status.can_transition_to(OrderStatus::Confirmed) {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status,
                to: OrderStatus::Confirmed,
            });
        }

        self.status = OrderStatus::Confirmed;
        self.updated_at = Utc::now();

        self.add_event(OrderEvent::OrderConfirmed {
            order_id: self.id,
            timestamp: self.updated_at,
        });

        Ok(())
    }

    /// Business logic: mark as paid
    pub fn mark_as_paid(&mut self, payment_id: Uuid) -> Result<(), DomainError> {
        if !self.status.can_transition_to(OrderStatus::Paid) {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status,
                to: OrderStatus::Paid,
            });
        }

        self.status = OrderStatus::Paid;
        self.updated_at = Utc::now();

        self.add_event(OrderEvent::OrderPaid {
            order_id: self.id,
            payment_id,
            timestamp: self.updated_at,
        });

        Ok(())
    }

    /// Business logic: ship the order
    pub fn ship(&mut self, tracking_number: String) -> Result<(), DomainError> {
        if !self.status.can_transition_to(OrderStatus::Shipped) {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status,
                to: OrderStatus::Shipped,
            });
        }

        self.status = OrderStatus::Shipped;
        self.updated_at = Utc::now();

        self.add_event(OrderEvent::OrderShipped {
            order_id: self.id,
            tracking_number,
            timestamp: self.updated_at,
        });

        Ok(())
    }

    /// Business logic: cancel the order
    pub fn cancel(&mut self, reason: String) -> Result<(), DomainError> {
        // Business rule: cannot cancel terminal orders
        if self.status.is_terminal() {
            return Err(DomainError::CannotCancelTerminalOrder);
        }

        if !self.status.can_transition_to(OrderStatus::Cancelled) {
            return Err(DomainError::InvalidStatusTransition {
                from: self.status,
                to: OrderStatus::Cancelled,
            });
        }

        self.status = OrderStatus::Cancelled;
        self.updated_at = Utc::now();

        self.add_event(OrderEvent::OrderCancelled {
            order_id: self.id,
            reason,
            timestamp: self.updated_at,
        });

        Ok(())
    }

    /// Business logic: add item (only in Pending status)
    pub fn add_item(&mut self, item: OrderItem) -> Result<(), DomainError> {
        if !self.status.can_be_modified() {
            return Err(DomainError::CannotModifyNonPendingOrder);
        }

        self.items.push(item);
        self.total = Self::calculate_total(&self.items)?;
        self.updated_at = Utc::now();

        Ok(())
    }

    /// Business logic: remove item (only in Pending status)
    pub fn remove_item(&mut self, item_id: crate::domain::value_objects::OrderItemId) -> Result<(), DomainError> {
        if !self.status.can_be_modified() {
            return Err(DomainError::CannotModifyNonPendingOrder);
        }

        let initial_len = self.items.len();
        self.items.retain(|item| item.id() != item_id);

        if self.items.len() == initial_len {
            return Err(DomainError::OrderItemNotFound);
        }

        if self.items.is_empty() {
            return Err(DomainError::CannotRemoveLastItem);
        }

        self.total = Self::calculate_total(&self.items)?;
        self.updated_at = Utc::now();

        Ok(())
    }

    /// Calculate total from items (business logic)
    fn calculate_total(items: &[OrderItem]) -> Result<Money, DomainError> {
        items
            .iter()
            .try_fold(Money::eur(Decimal::ZERO)?, |acc, item| {
                (acc + item.subtotal()).map_err(DomainError::from)
            })
    }

    // Getters (encapsulation)
    pub fn id(&self) -> OrderId {
        self.id
    }

    pub fn customer_id(&self) -> CustomerId {
        self.customer_id
    }

    pub fn status(&self) -> OrderStatus {
        self.status
    }

    pub fn total(&self) -> Money {
        self.total
    }

    pub fn items(&self) -> &[OrderItem] {
        &self.items
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // Domain Events management
    fn add_event(&mut self, event: OrderEvent) {
        self.domain_events.push(event);
    }

    pub fn take_events(&mut self) -> Vec<OrderEvent> {
        std::mem::take(&mut self.domain_events)
    }

    pub fn events(&self) -> &[OrderEvent] {
        &self.domain_events
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{Currency, ProductId};

    fn create_test_item() -> OrderItem {
        OrderItem::new(
            ProductId::new(),
            "Test Product".to_string(),
            1,
            Money::eur(Decimal::new(1000, 2)).unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn test_order_creation() {
        let items = vec![create_test_item()];
        let order = Order::create(CustomerId::new(), items).unwrap();

        assert_eq!(order.status(), OrderStatus::Pending);
        assert_eq!(order.items().len(), 1);
        assert!(!order.events().is_empty());
    }

    #[test]
    fn test_empty_order_fails() {
        let result = Order::create(CustomerId::new(), vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_order_confirmation() {
        let items = vec![create_test_item()];
        let mut order = Order::create(CustomerId::new(), items).unwrap();

        order.confirm().unwrap();
        assert_eq!(order.status(), OrderStatus::Confirmed);
    }

    #[test]
    fn test_invalid_state_transition() {
        let items = vec![create_test_item()];
        let mut order = Order::create(CustomerId::new(), items).unwrap();

        // Cannot go directly from Pending to Shipped
        let result = order.ship("TRACK123".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_modify_confirmed_order() {
        let items = vec![create_test_item()];
        let mut order = Order::create(CustomerId::new(), items).unwrap();
        order.confirm().unwrap();

        let result = order.add_item(create_test_item());
        assert!(result.is_err());
    }
}
