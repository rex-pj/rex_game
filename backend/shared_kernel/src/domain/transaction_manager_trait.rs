use std::any::Any;

use crate::domain::errors::domain_error::DomainError;

pub trait TransactionWrapperTrait {
    fn as_any(&self) -> &dyn Any;
}

pub trait TransactionManagerTrait {
    type TransactionWrapper: TransactionWrapperTrait;

    fn commit(&self, tx: Self::TransactionWrapper) -> impl std::future::Future<Output = Result<(), DomainError>> + Send;
    fn rollback(&self, tx: Self::TransactionWrapper) -> impl std::future::Future<Output = Result<(), DomainError>> + Send;
    fn begin(&self) -> impl std::future::Future<Output = Result<Self::TransactionWrapper, DomainError>> + Send;
}
