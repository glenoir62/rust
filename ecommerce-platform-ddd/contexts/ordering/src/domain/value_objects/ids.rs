use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Type-safe IDs to prevent mixing different entity IDs
macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            pub fn from_uuid(uuid: Uuid) -> Self {
                Self(uuid)
            }

            pub fn value(&self) -> Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

define_id!(OrderId);
define_id!(OrderItemId);
define_id!(CustomerId);
define_id!(ProductId);
define_id!(PaymentId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_id_creation() {
        let id1 = OrderId::new();
        let id2 = OrderId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_order_id_from_uuid() {
        let uuid = Uuid::new_v4();
        let id = OrderId::from_uuid(uuid);
        assert_eq!(id.value(), uuid);
    }
}
