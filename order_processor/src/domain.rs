
use crate::{OrderId, CustomerId, Money, MoneyError};
use crate::parsing::OrderLineOwned;

#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderId,
    pub customer_id: CustomerId,
    pub created_at_epoch_s: i64,
    pub currency: crate::Currency,
    pub lines: Vec<OrderLineOwned>,
}

impl Order {
    pub fn total(&self) -> Result<Money, MoneyError> {
        let mut acc = Money::zero(self.currency);
        for l in &self.lines {
            let line_total = Money { cents: (l.unit.cents * l.qty.get() as i64), currency: self.currency };
            acc = acc.add(line_total)?;
        }
        Ok(acc)
    }
}
