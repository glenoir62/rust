pub mod application;
pub mod domain;
pub mod infrastructure;

// Re-export commonly used types
pub use domain::{Order, OrderEvent, OrderItem, OrderRepository};
