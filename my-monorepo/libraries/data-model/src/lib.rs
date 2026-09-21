//! data-model: Tipos de domínio compartilhados entre serviços.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Erros de validação e domínio.
#[derive(Debug, Error)]
pub enum ModelError {
    #[error("invalid name: {0}")]
    InvalidName(String),
}

pub type Result<T> = std::result::Result<T, ModelError>;

/// Payload para criar um usuário.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NewUser {
    pub name: String,
}

impl NewUser {
    /// Regras simples de validação para exemplo.
    pub fn validate(&self) -> Result<()> {
        let trimmed = self.name.trim();
        if trimmed.len() < 3 {
            return Err(ModelError::InvalidName(
                "name must be at least 3 chars".into(),
            ));
        }
        if trimmed.chars().any(|c| c.is_control()) {
            return Err(ModelError::InvalidName("control chars not allowed".into()));
        }
        Ok(())
    }
}

/// Entidade persistida/retornada.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: u64,
    pub name: String,
}

impl User {
    /// Constrói um `User` a partir do `NewUser` e de um id gerado externamente.
    pub fn from_new(new: NewUser, id: u64) -> Result<Self> {
        new.validate()?;
        Ok(Self { id, name: new.name })
    }
}

/// Re-export comum em binários/tests.
pub mod prelude {
    pub use crate::{ModelError, NewUser, Result, User};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_new_user() {
        let n = NewUser { name: "Alice".into() };
        assert!(n.validate().is_ok());
        let u = User::from_new(n, 42).unwrap();
        assert_eq!(u.id, 42);
        assert_eq!(u.name, "Alice");
    }

    #[test]
    fn invalid_new_user() {
        let n = NewUser { name: "  x ".into() };
        assert!(matches!(n.validate(), Err(ModelError::InvalidName(_))));
    }
}
