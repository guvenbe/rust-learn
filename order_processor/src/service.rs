use crate::{OrderRepository, Notifier, AuditLogger, ProcessingError, parse_order, validate_order, parse_lines};
use crate::{Order, OrderId};

/// Application service with explicit dependencies and small steps.
pub struct OrderService<R: OrderRepository, N: Notifier, L: AuditLogger> {
    repo: R,
    notifier: N,
    logger: L,
}

impl<R: OrderRepository, N: Notifier, L: AuditLogger> OrderService<R, N, L> {
    pub fn new(repo: R, notifier: N, logger: L) -> Self { Self { repo, notifier, logger } }

    /// Parse -> validate -> transform -> store -> side-effects.
    pub fn place_order(&mut self, input: &str) -> Result<Order, ProcessingError> {
        self.logger.log("parsing order input");
        let p = parse_order(input)?;
        self.logger.log("validating order input");
        let v = validate_order(p)?;
        self.logger.log("parsing order lines");
        let lines = parse_lines(v.lines, v.currency)?;
        let order = Order { id: v.id, customer_id: v.customer_id, created_at_epoch_s: chrono::Utc::now().timestamp(), currency: v.currency, lines };
        self.logger.log("storing order");
        self.repo.insert(order.clone()).map_err(|_| crate::RepositoryError::Access).unwrap();
        self.notifier.notify("order placed");
        Ok(order)
    }

    pub fn get(&self, id: OrderId) -> Result<Option<Order>, crate::RepositoryError> { self.repo.get(id) }
}
