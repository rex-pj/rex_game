use std::any::Any;

use crate::InfraError;

pub trait TransactionWrapperTrait {
    fn as_any(&self) -> &dyn Any;
}

pub trait TransactionManagerTrait {
    type TransactionWrapper: TransactionWrapperTrait;

    fn commit(
        &self,
        tx: Self::TransactionWrapper,
    ) -> impl std::future::Future<Output = Result<(), InfraError>> + Send;
    fn rollback(
        &self,
        tx: Self::TransactionWrapper,
    ) -> impl std::future::Future<Output = Result<(), InfraError>> + Send;
    fn begin(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::TransactionWrapper, InfraError>> + Send;
}
