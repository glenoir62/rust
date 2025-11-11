use crate::domain::value_objects::{MoneyError, OrderStatus};
use thiserror::Error;

/// Domain-specific errors
/// These represent business rule violations
#[derive(Debug, Error)]
pub enum DomainError {
    // Order errors
    #[error("Order cannot be empty")]
    EmptyOrder,

    #[error("Cannot transition from {from:?} to {to:?}")]
    InvalidStatusTransition {
        from: OrderStatus,
        to: OrderStatus,
    },

    #[error("Cannot cancel an order in terminal state")]
    CannotCancelTerminalOrder,

    #[error("Cannot modify order that is not in pending status")]
    CannotModifyNonPendingOrder,

    #[error("Order item not found")]
    OrderItemNotFound,

    #[error("Cannot remove the last item from an order")]
    CannotRemoveLastItem,

    // Order item errors
    #[error("Quantity must be greater than zero")]
    InvalidQuantity,

    #[error("Product name cannot be empty")]
    InvalidProductName,

    // Money errors
    #[error("Money error: {0}")]
    MoneyError(#[from] MoneyError),

    // Repository errors
    #[error("Order not found")]
    OrderNotFound,

    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl From<sea_orm::DbErr> for DomainError {
    fn from(err: sea_orm::DbErr) -> Self {
        DomainError::DatabaseError(err.to_string())
    }
}
