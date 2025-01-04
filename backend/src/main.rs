use config::{Config, File};

use app_state::RegularAppState;
use axum::{routing::get, Router};
use handlers::flashcard_handler::FlashcardHandler;
use rex_game_application::flashcards::flashcard_usecase::FlashcardUseCase;
use rex_game_infrastructure::{
    repositories::flashcard_repository::FlashcardRepository, seaorm_connection::SeaOrmConnection,
};
use tokio::net::TcpListener;
pub mod app_state;
pub mod handlers;

fn build_routers(app_state: RegularAppState) -> Router {
    Router::new().route(
        "/flash-cards",
        get(FlashcardHandler::get_flashcards::<RegularAppState>).with_state(app_state),
    )
}

#[tokio::main]
async fn start() {
    let connection_str = get_connection_string();
    let db_connection = SeaOrmConnection::new(&connection_str).await;
    match db_connection {
        Ok(connection) => {
            println!("Successfully connected to the database.");
            let flashcard_repository = FlashcardRepository::new(connection.pool);
            let flashcard_usecase = FlashcardUseCase::new(flashcard_repository.clone());
            let app_state = RegularAppState {
                flashcard_repository: flashcard_repository.clone(),
                flashcard_usecase,
            };

            let app = build_routers(app_state);
            let listener = TcpListener::bind("0.0.0.0:3400").await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }
        Err(err) => {
            eprintln!("Failed to connect to the database: {:?}", err);
        }
    }
}

fn get_connection_string() -> String {
    let config_file = File::with_name("src/config.toml");
    let settings = Config::builder()
        .add_source(config_file)
        .build()
        .expect("Failed to load configuration");

    settings
        .get_string("database.url")
        .expect("Database URL is missing")
}

fn main() {
    start()
}
