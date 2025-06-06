use crate::errors::domain_error::DomainError;
use std::{any::Any, future::Future};

pub trait TransactionWrapperTrait: Send + Sync + Any {
    fn as_any(&self) -> &dyn Any;
}

pub trait TransactionManagerTrait {
    type TransactionWrapper: TransactionWrapperTrait + Send + Sync;
    fn begin(&self) -> impl Future<Output = Result<Self::TransactionWrapper, DomainError>>;
    fn commit(
        &self,
        tx: Self::TransactionWrapper,
    ) -> impl Future<Output = Result<(), DomainError>> + Send;
    fn rollback(
        &self,
        tx: Self::TransactionWrapper,
    ) -> impl Future<Output = Result<(), DomainError>> + Send;
}
