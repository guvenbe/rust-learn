use std::collections::HashMap;
use crate::{Order, OrderId, RepositoryError};

/// Explicit dependency by trait. No hidden globals.
pub trait OrderRepository {
    fn insert(&mut self, order: Order) -> Result<(), RepositoryError>;
    fn get(&self, id: OrderId) -> Result<Option<Order>, RepositoryError>;
}

#[derive(Default)]
pub struct InMemoryOrderRepo { map: HashMap<OrderId, Order> }

impl InMemoryOrderRepo { pub fn new() -> Self { Self { map: HashMap::new() } } }

impl OrderRepository for InMemoryOrderRepo {
    fn insert(&mut self, order: Order) -> Result<(), RepositoryError> {
        self.map.insert(order.id, order); Ok(())
    }
    fn get(&self, id: OrderId) -> Result<Option<Order>, RepositoryError> {
        Ok(self.map.get(&id).cloned())
    }
}
