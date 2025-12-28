use crate::domain::errors::domain_error::{DomainError, ErrorType};
use crate::domain::transaction_manager_trait::{TransactionManagerTrait, TransactionWrapperTrait};
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use std::any::Any;
use std::sync::Arc;

#[derive(Clone)]
pub struct TransactionManager {
    _db_connection: Arc<DatabaseConnection>,
}

impl TransactionManager {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl TransactionManagerTrait for TransactionManager {
    type TransactionWrapper = SeaOrmTransactionWrapper;

    async fn begin(&self) -> Result<Self::TransactionWrapper, DomainError> {
        let txn = self
            ._db_connection
            .begin()
            .await
            .map_err(|e| DomainError::new(ErrorType::DatabaseError, &e.to_string(), None))?;
        Ok(SeaOrmTransactionWrapper { txn: Some(txn) })
    }

    async fn commit(&self, mut tx: Self::TransactionWrapper) -> Result<(), DomainError> {
        if let Some(txn) = tx.txn.take() {
            txn.commit()
                .await
                .map_err(|e| DomainError::new(ErrorType::DatabaseError, &e.to_string(), None))
        } else {
            Err(DomainError::new(
                ErrorType::DatabaseError,
                "Transaction already consumed",
                None,
            ))
        }
    }

    async fn rollback(&self, mut tx: Self::TransactionWrapper) -> Result<(), DomainError> {
        if let Some(txn) = tx.txn.take() {
            txn.rollback()
                .await
                .map_err(|e| DomainError::new(ErrorType::DatabaseError, &e.to_string(), None))
        } else {
            Err(DomainError::new(
                ErrorType::DatabaseError,
                "Transaction already consumed",
                None,
            ))
        }
    }
}

pub struct SeaOrmTransactionWrapper {
    pub txn: Option<DatabaseTransaction>,
}

impl SeaOrmTransactionWrapper {
    pub fn get_transaction(&mut self) -> &mut DatabaseTransaction {
        self.txn.as_mut().expect("Transaction already consumed")
    }
}

impl TransactionWrapperTrait for SeaOrmTransactionWrapper {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
