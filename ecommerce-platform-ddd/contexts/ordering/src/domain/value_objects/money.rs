use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

/// Money Value Object
/// Immutable, self-validating
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    amount: Decimal,
    currency: Currency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    EUR,
    USD,
    GBP,
}

impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Result<Self, MoneyError> {
        if amount < Decimal::ZERO {
            return Err(MoneyError::NegativeAmount);
        }
        Ok(Self { amount, currency })
    }
    
    pub fn eur(amount: Decimal) -> Result<Self, MoneyError> {
        Self::new(amount, Currency::EUR)
    }
    
    pub fn usd(amount: Decimal) -> Result<Self, MoneyError> {
        Self::new(amount, Currency::USD)
    }
    
    pub fn amount(&self) -> Decimal {
        self.amount
    }
    
    pub fn currency(&self) -> Currency {
        self.currency
    }
    
    pub fn is_zero(&self) -> bool {
        self.amount == Decimal::ZERO
    }
}

// Arithmetic operations with currency validation
impl Add for Money {
    type Output = Result<Money, MoneyError>;
    
    fn add(self, other: Self) -> Self::Output {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch);
        }
        Ok(Money {
            amount: self.amount + other.amount,
            currency: self.currency,
        })
    }
}

impl Sub for Money {
    type Output = Result<Money, MoneyError>;
    
    fn sub(self, other: Self) -> Self::Output {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch);
        }
        let result = self.amount - other.amount;
        Money::new(result, self.currency)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MoneyError {
    #[error("Amount cannot be negative")]
    NegativeAmount,
    
    #[error("Currency mismatch in operation")]
    CurrencyMismatch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_creation() {
        let money = Money::eur(Decimal::new(100, 0)).unwrap();
        assert_eq!(money.amount(), Decimal::new(100, 0));
        assert_eq!(money.currency(), Currency::EUR);
    }

    #[test]
    fn test_negative_money_fails() {
        let result = Money::eur(Decimal::new(-10, 0));
        assert!(result.is_err());
    }

    #[test]
    fn test_money_addition() {
        let m1 = Money::eur(Decimal::new(100, 0)).unwrap();
        let m2 = Money::eur(Decimal::new(50, 0)).unwrap();
        let result = (m1 + m2).unwrap();
        assert_eq!(result.amount(), Decimal::new(150, 0));
    }

    #[test]
    fn test_currency_mismatch() {
        let m1 = Money::eur(Decimal::new(100, 0)).unwrap();
        let m2 = Money::usd(Decimal::new(50, 0)).unwrap();
        let result = m1 + m2;
        assert!(result.is_err());
    }
}
