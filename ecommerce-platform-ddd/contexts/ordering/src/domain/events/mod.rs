use crate::domain::value_objects::{CustomerId, Money, OrderId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Domain Events - Immutable records of things that happened
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderEvent {
    OrderCreated {
        order_id: OrderId,
        customer_id: CustomerId,
        total: Money,
        timestamp: DateTime<Utc>,
    },
    OrderConfirmed {
        order_id: OrderId,
        timestamp: DateTime<Utc>,
    },
    OrderPaid {
        order_id: OrderId,
        payment_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    OrderShipped {
        order_id: OrderId,
        tracking_number: String,
        timestamp: DateTime<Utc>,
    },
    OrderDelivered {
        order_id: OrderId,
        timestamp: DateTime<Utc>,
    },
    OrderCancelled {
        order_id: OrderId,
        reason: String,
        timestamp: DateTime<Utc>,
    },
}

impl OrderEvent {
    /// Get the order ID for any event type
    pub fn order_id(&self) -> OrderId {
        match self {
            OrderEvent::OrderCreated { order_id, .. }
            | OrderEvent::OrderConfirmed { order_id, .. }
            | OrderEvent::OrderPaid { order_id, .. }
            | OrderEvent::OrderShipped { order_id, .. }
            | OrderEvent::OrderDelivered { order_id, .. }
            | OrderEvent::OrderCancelled { order_id, .. } => *order_id,
        }
    }

    /// Get the timestamp for any event type
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            OrderEvent::OrderCreated { timestamp, .. }
            | OrderEvent::OrderConfirmed { timestamp, .. }
            | OrderEvent::OrderPaid { timestamp, .. }
            | OrderEvent::OrderShipped { timestamp, .. }
            | OrderEvent::OrderDelivered { timestamp, .. }
            | OrderEvent::OrderCancelled { timestamp, .. } => *timestamp,
        }
    }

    /// Get a human-readable event name
    pub fn event_name(&self) -> &'static str {
        match self {
            OrderEvent::OrderCreated { .. } => "ORDER_CREATED",
            OrderEvent::OrderConfirmed { .. } => "ORDER_CONFIRMED",
            OrderEvent::OrderPaid { .. } => "ORDER_PAID",
            OrderEvent::OrderShipped { .. } => "ORDER_SHIPPED",
            OrderEvent::OrderDelivered { .. } => "ORDER_DELIVERED",
            OrderEvent::OrderCancelled { .. } => "ORDER_CANCELLED",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_event_serialization() {
        let event = OrderEvent::OrderCreated {
            order_id: OrderId::new(),
            customer_id: CustomerId::new(),
            total: Money::eur(Decimal::new(10000, 2)).unwrap(),
            timestamp: Utc::now(),
        };

        let json = serde_json::to_string(&event).unwrap();
        let deserialized: OrderEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.order_id(), deserialized.order_id());
    }
}
