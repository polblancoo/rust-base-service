use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Error connecting to database: {0}")]
    ConnectionError(String),
    
    #[error("Query execution error: {0}")]
    QueryError(String),
    
    #[error("Entity not found: {0}")]
    NotFoundError(String),
    
    #[error("Constraint violation: {0}")]
    ConstraintError(String),
}
