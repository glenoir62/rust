use serde::{Deserialize, Serialize};

/// OrderStatus Value Object
/// Encapsulates valid status transitions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Paid,
    Shipped,
    Delivered,
    Cancelled,
}

impl OrderStatus {
    /// Business rule: valid state transitions
    pub fn can_transition_to(&self, new_status: OrderStatus) -> bool {
        use OrderStatus::*;
        matches!(
            (self, new_status),
            (Pending, Confirmed)
                | (Confirmed, Paid)
                | (Paid, Shipped)
                | (Shipped, Delivered)
                | (Pending, Cancelled)
                | (Confirmed, Cancelled)
        )
    }
    
    pub fn is_terminal(&self) -> bool {
        matches!(self, OrderStatus::Delivered | OrderStatus::Cancelled)
    }
    
    pub fn can_be_modified(&self) -> bool {
        matches!(self, OrderStatus::Pending)
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "PENDING"),
            OrderStatus::Confirmed => write!(f, "CONFIRMED"),
            OrderStatus::Paid => write!(f, "PAID"),
            OrderStatus::Shipped => write!(f, "SHIPPED"),
            OrderStatus::Delivered => write!(f, "DELIVERED"),
            OrderStatus::Cancelled => write!(f, "CANCELLED"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        assert!(OrderStatus::Pending.can_transition_to(OrderStatus::Confirmed));
        assert!(OrderStatus::Confirmed.can_transition_to(OrderStatus::Paid));
        assert!(OrderStatus::Paid.can_transition_to(OrderStatus::Shipped));
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(!OrderStatus::Pending.can_transition_to(OrderStatus::Delivered));
        assert!(!OrderStatus::Cancelled.can_transition_to(OrderStatus::Paid));
    }

    #[test]
    fn test_terminal_states() {
        assert!(OrderStatus::Delivered.is_terminal());
        assert!(OrderStatus::Cancelled.is_terminal());
        assert!(!OrderStatus::Pending.is_terminal());
    }
}
