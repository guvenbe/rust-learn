/// Small, explicit newtypes make intent and invariants clear.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomerId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub u64);

#[derive(Debug, Clone)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn new(s: &str, field: &'static str) -> Result<Self, crate::ValidationError> {
        let trimmed = s.trim();
        if trimmed.is_empty() { return Err(crate::ValidationError::Empty(field)); }
        Ok(Self(trimmed.to_owned()))
    }
    pub fn as_str(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency { BRL, USD, EUR }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money { pub cents: i64, pub currency: Currency }

impl Money {
    pub fn zero(currency: Currency) -> Self { Self { cents: 0, currency } }
    pub fn from_units(units: i64, currency: Currency) -> Self { Self { cents: units * 100, currency } }
    pub fn add(self, other: Money) -> Result<Money, crate::MoneyError> {
        if self.currency != other.currency { return Err(crate::MoneyError::CurrencyMismatch); }
        Ok(Money { cents: self.cents + other.cents, currency: self.currency })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositiveQuantity(u32);
impl PositiveQuantity {
    pub fn new(v: u32) -> Result<Self, crate::ValidationError> {
        if v == 0 { Err(crate::ValidationError::NonPositiveQuantity) } else { Ok(Self(v)) }
    }
    pub fn get(self) -> u32 { self.0 }
}
