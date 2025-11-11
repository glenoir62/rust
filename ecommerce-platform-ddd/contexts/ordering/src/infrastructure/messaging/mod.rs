use crate::domain::{events::OrderEvent, errors::DomainError};
use async_trait::async_trait;

/// Trait for publishing domain events
#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: OrderEvent) -> Result<(), DomainError>;
}

/// Iggy implementation (à compléter avec vraie connexion Iggy)
pub struct IggyEventPublisher {
    // client: IggyClient
}

impl IggyEventPublisher {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl EventPublisher for IggyEventPublisher {
    async fn publish(&self, event: OrderEvent) -> Result<(), DomainError> {
        // TODO: Implement actual Iggy publishing
        tracing::info!("Publishing event: {:?}", event);
        Ok(())
    }
}

/// No-op publisher for testing
pub struct NoOpEventPublisher;

#[async_trait]
impl EventPublisher for NoOpEventPublisher {
    async fn publish(&self, _event: OrderEvent) -> Result<(), DomainError> {
        Ok(())
    }
}
