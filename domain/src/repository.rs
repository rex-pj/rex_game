use std::future::Future;

use crate::Entity;

#[derive(Debug)]
pub enum RepoGetError {
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoCreateError {
    InvalidData(String),
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoFindError {
    NotFound,
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoUpdateError {
    NotFound,
    InvalidData(String),
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoDeleteError {
    NotFound,
    InvalidData(String),
    Unknown(String),
}

pub trait Repository<T>: Send + Sync
where
    T: Entity,
{
    /// Insert the received entity in the persistence system
    fn create<'a>(&'a self, data: T) -> impl Future<Output = Result<T, RepoCreateError>> + Send;

    /// Find and return one single record from the persistence system
    fn find(&self, id: i32) -> impl Future<Output = Result<T, RepoFindError>> + Send;

    /// Find and return all records corresponding to the search criteria from the persistence system
    fn get<F>(&self, f: F) -> impl Future<Output = Result<Vec<T>, RepoGetError>> + Send
    where
        F: Fn(T) -> T + Send;

    /// Update one single record already present in the persistence system
    fn update(&self, data: T) -> impl Future<Output = Result<T, RepoUpdateError>> + Send;

    /// Delete one single record from the persistence system
    fn delete(&self, id: i32) -> impl Future<Output = Result<(), RepoDeleteError>> + Send;
}
