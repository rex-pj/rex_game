use crate::domain::transaction_manager_trait::{TransactionManagerTrait, TransactionWrapperTrait};
use crate::InfraError;
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

    async fn begin(&self) -> Result<Self::TransactionWrapper, InfraError> {
        let txn = self
            ._db_connection
            .begin()
            .await
            .map_err(|e| InfraError::database(&e.to_string()))?;
        Ok(SeaOrmTransactionWrapper { txn: Some(txn) })
    }

    async fn commit(&self, mut tx: Self::TransactionWrapper) -> Result<(), InfraError> {
        if let Some(txn) = tx.txn.take() {
            txn.commit()
                .await
                .map_err(|e| InfraError::database(&e.to_string()))
        } else {
            Err(InfraError::database("Transaction already consumed"))
        }
    }

    async fn rollback(&self, mut tx: Self::TransactionWrapper) -> Result<(), InfraError> {
        if let Some(txn) = tx.txn.take() {
            txn.rollback()
                .await
                .map_err(|e| InfraError::database(&e.to_string()))
        } else {
            Err(InfraError::database("Transaction already consumed"))
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
