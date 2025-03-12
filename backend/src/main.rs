pub mod app_state;
pub mod handlers;
pub mod helpers;
pub mod middlewares;
pub mod routings;
pub mod startup;
pub mod view_models;

fn main() {
    startup::start()
}
