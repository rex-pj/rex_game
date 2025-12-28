pub mod authenticate_middleware;
pub mod authorize_middleware;
pub mod rate_limit_middleware;
pub mod error_handler_middleware;

#[derive(Debug, Clone)]
pub enum AuthorizedState {
    IsInRole = 1,
    HasPermission = 2,
}
