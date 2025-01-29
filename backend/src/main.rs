use app_state::RegularAppState;
use axum::{routing::get, Router};
use config::{Config, File};
use handlers::flashcard_handler::FlashcardHandler;
use handlers::flashcard_type_handler::FlashcardTypeHandler;
use rex_game_application::{
    flashcard_types::flashcard_type_usecase::FlashcardTypeUseCase,
    flashcards::flashcard_usecase::FlashcardUseCase,
};
use rex_game_infrastructure::{
    repositories::{
        flashcard_file_repository::FlashcardFileRepository,
        flashcard_repository::FlashcardRepository,
        flashcard_type_relation_repository::FlashcardTypeRelationRepository,
        flashcard_type_repository::FlashcardTypeRepository,
    },
    seaorm_connection::SeaOrmConnection,
};
use tokio::net::TcpListener;
pub mod app_state;
pub mod handlers;
pub mod helpers;

fn build_routers(app_state: RegularAppState) -> Router {
    Router::new()
        .route(
            "/flash-cards",
            get(FlashcardHandler::get_flashcards::<RegularAppState>)
                .post(FlashcardHandler::create_flashcard::<RegularAppState>),
        )
        .route(
            "/flash-cards/:id",
            get(FlashcardHandler::get_flashcard_by_id::<RegularAppState>)
                .patch(FlashcardHandler::update_flashcard::<RegularAppState>),
        )
        .route(
            "/flash-cards/images/:id",
            get(FlashcardHandler::get_flashcard_image::<RegularAppState>),
        )
        .route(
            "/flash-card-types",
            get(FlashcardTypeHandler::get_flashcard_types::<RegularAppState>)
                .post(FlashcardTypeHandler::create_flashcard_type::<RegularAppState>),
        )
        .route(
            "/flash-card-types/:id",
            get(FlashcardTypeHandler::get_flashcard_type_by_id::<RegularAppState>)
                .put(FlashcardTypeHandler::update_flashcard_type::<RegularAppState>),
        )
        .with_state(app_state)
}

#[tokio::main]
async fn start() {
    let connection_str = get_connection_string();
    let db_connection = SeaOrmConnection::new(&connection_str).await;
    match db_connection {
        Ok(connection) => {
            println!("Successfully connected to the database.");
            let flashcard_repository = FlashcardRepository::new(connection.pool.clone());
            let flashcard_file_repository = FlashcardFileRepository::new(connection.pool.clone());
            let flashcard_type_relation_repository =
                FlashcardTypeRelationRepository::new(connection.pool.clone());

            let flashcard_usecase = FlashcardUseCase::new(
                flashcard_repository,
                flashcard_file_repository,
                flashcard_type_relation_repository,
            );

            let flashcard_type_repository = FlashcardTypeRepository::new(connection.pool.clone());
            let flashcard_type_usecase = FlashcardTypeUseCase::new(flashcard_type_repository);
            let app_state = RegularAppState {
                flashcard_usecase,
                flashcard_type_usecase,
            };

            let app = build_routers(app_state);
            let listener = TcpListener::bind("0.0.0.0:3400").await.unwrap();
            println!("The application is running at: http://localhost:3400");
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
