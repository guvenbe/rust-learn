use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("missing field: {0}")]
    MissingField(&'static str),
    #[error("invalid format")]
    InvalidFormat,
    #[error("invalid number in field: {0}")]
    InvalidNumber(&'static str),
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("empty string in field: {0}")]
    Empty(&'static str),
    #[error("invalid email format")]
    InvalidEmail,
    #[error("non-positive quantity")]
    NonPositiveQuantity,
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("repository access failure")]
    Access,
}

#[derive(Debug, Error)]
pub enum MoneyError {
    #[error("currency mismatch")]
    CurrencyMismatch,
}

#[derive(Debug, Error)]
pub enum ProcessingError {
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error(transparent)]
    Validate(#[from] ValidationError),
    #[error(transparent)]
    Money(#[from] MoneyError),
}
