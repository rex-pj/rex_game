use app_state::{AppState, RegularAppState};
use axum::{routing::get, Router};
use handlers::flashcard_handler::FlashcardHandler;
use rex_game_application::flashcards::flashcard_usecase::FlashcardUseCase;
use rex_game_infrastructure::repositories::flashcard_repository::FlashcardRepository;
use tokio::net::TcpListener;
pub mod app_state;
pub mod handlers;

fn build<S: AppState>(state: S) -> Router {
    Router::new().route(
        "/",
        get(FlashcardHandler::get_flashcard::<S>).with_state(state),
    )
}

#[tokio::main]
// #[debug_handler]
async fn start() {
    let flashcard_repository = FlashcardRepository {};
    let flashcard_usecase = FlashcardUseCase::new(flashcard_repository);

    let app_state = RegularAppState {
        flashcard_repository,
        flashcard_usecase,
    };

    let app = build(app_state);
    let listener = TcpListener::bind("0.0.0.0:3400").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn main() {
    start();
}
