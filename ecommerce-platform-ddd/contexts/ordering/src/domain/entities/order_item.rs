use crate::domain::value_objects::{Money, OrderItemId, ProductId};
use crate::domain::errors::DomainError;
use rust_decimal::Decimal;

/// OrderItem Entity
/// Part of the Order aggregate, not an aggregate root itself
#[derive(Debug, Clone)]
pub struct OrderItem {
    id: OrderItemId,
    product_id: ProductId,
    product_name: String,
    quantity: u32,
    unit_price: Money,
}

impl OrderItem {
    /// Factory method with validation
    pub fn new(
        product_id: ProductId,
        product_name: String,
        quantity: u32,
        unit_price: Money,
    ) -> Result<Self, DomainError> {
        // Business rule: quantity must be positive
        if quantity == 0 {
            return Err(DomainError::InvalidQuantity);
        }

        // Business rule: product name cannot be empty
        if product_name.trim().is_empty() {
            return Err(DomainError::InvalidProductName);
        }

        Ok(Self {
            id: OrderItemId::new(),
            product_id,
            product_name,
            quantity,
            unit_price,
        })
    }

    /// Business logic: calculate subtotal
    pub fn subtotal(&self) -> Money {
        let amount = self.unit_price.amount() * Decimal::from(self.quantity);
        Money::new(amount, self.unit_price.currency())
            .expect("Subtotal calculation should always produce valid money")
    }

    /// Change quantity (business rule: must remain positive)
    pub fn change_quantity(&mut self, new_quantity: u32) -> Result<(), DomainError> {
        if new_quantity == 0 {
            return Err(DomainError::InvalidQuantity);
        }
        self.quantity = new_quantity;
        Ok(())
    }

    // Getters
    pub fn id(&self) -> OrderItemId {
        self.id
    }

    pub fn product_id(&self) -> ProductId {
        self.product_id
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn unit_price(&self) -> Money {
        self.unit_price
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::Currency;

    #[test]
    fn test_order_item_creation() {
        let item = OrderItem::new(
            ProductId::new(),
            "Product A".to_string(),
            2,
            Money::eur(Decimal::new(1000, 2)).unwrap(),
        )
        .unwrap();

        assert_eq!(item.quantity(), 2);
        assert_eq!(item.product_name(), "Product A");
    }

    #[test]
    fn test_zero_quantity_fails() {
        let result = OrderItem::new(
            ProductId::new(),
            "Product A".to_string(),
            0,
            Money::eur(Decimal::new(1000, 2)).unwrap(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_subtotal_calculation() {
        let item = OrderItem::new(
            ProductId::new(),
            "Product A".to_string(),
            3,
            Money::eur(Decimal::new(1000, 2)).unwrap(), // 10.00 EUR
        )
        .unwrap();

        let subtotal = item.subtotal();
        assert_eq!(subtotal.amount(), Decimal::new(3000, 2)); // 30.00 EUR
    }
}
