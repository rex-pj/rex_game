use std::{any::Any, sync::Arc};

use rex_game_domain::{
    errors::domain_error::{DomainError, ErrorType},
    transaction_manager_trait::{TransactionManagerTrait, TransactionWrapperTrait},
};
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};

pub struct SeaOrmTransactionWrapper {
    pub transaction: DatabaseTransaction,
}

impl TransactionWrapperTrait for SeaOrmTransactionWrapper {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct TransactionManager {
    db: Arc<DatabaseConnection>,
}

impl TransactionManager {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

impl TransactionManagerTrait for TransactionManager {
    type TransactionWrapper = SeaOrmTransactionWrapper;
    async fn commit(&self, tx: Self::TransactionWrapper) -> Result<(), DomainError> {
        tx.transaction.commit().await.map_err(|_| {
            DomainError::new(
                ErrorType::DatabaseError,
                "Failed to commit transaction",
                None,
            )
        })
    }

    async fn rollback(&self, tx: Self::TransactionWrapper) -> Result<(), DomainError> {
        tx.transaction.rollback().await.map_err(|_| {
            DomainError::new(
                ErrorType::DatabaseError,
                "Failed to rollback transaction",
                None,
            )
        })
    }

    async fn begin(&self) -> Result<Self::TransactionWrapper, DomainError> {
        let transaction = self.db.begin().await.map_err(|err| {
            DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
        })?;

        Ok(Self::TransactionWrapper { transaction })
    }
}
