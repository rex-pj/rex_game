pub mod authenticate_middleware;
pub mod permission_middleware;

#[derive(Debug, Clone)]
pub enum AuthorizedState {
    IsInRole = 1,
    HasPermission = 2,
}
