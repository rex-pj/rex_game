use app_state::RegularAppState;
use axum::{routing::get, routing::post, Router};
use handlers::flashcard_type_handler::FlashcardTypeHandler;
use handlers::user_handler::UserHandler;
use handlers::{
    authentication_handler::AuthenticationHandler, flashcard_handler::FlashcardHandler,
};
use rex_game_application::identities::identity_login_usecase::IdentityLoginUseCase;
use rex_game_application::identities::identity_user_usecase::IdentityUserUseCase;
use rex_game_application::{
    flashcard_types::flashcard_type_usecase::FlashcardTypeUseCase,
    flashcards::flashcard_usecase::FlashcardUseCase, users::user_usecase::UserUseCase,
};
use rex_game_infrastructure::helpers::configuration_helper::ConfigurationHelper;
use rex_game_infrastructure::identities::identity_password_hasher::IdentityPasswordHasher;
use rex_game_infrastructure::identities::identity_token_helper::IdentityTokenHelper;
use rex_game_infrastructure::{
    repositories::{
        flashcard_file_repository::FlashcardFileRepository,
        flashcard_repository::FlashcardRepository,
        flashcard_type_relation_repository::FlashcardTypeRelationRepository,
        flashcard_type_repository::FlashcardTypeRepository, user_repository::UserRepository,
    },
    seaorm_connection::SeaOrmConnection,
};
use tokio::net::TcpListener;
pub mod app_state;
pub mod handlers;
pub mod helpers;
pub mod view_models;

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
        .route("/users", post(UserHandler::create_user::<RegularAppState>))
        .route(
            "/auth/authenticate",
            post(AuthenticationHandler::authenticate::<RegularAppState>),
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
            let user_repository = UserRepository::new(connection.pool.clone());
            let identity_password_hasher = IdentityPasswordHasher::new();
            let identity_token_helper = IdentityTokenHelper::new();

            let flashcard_usecase = FlashcardUseCase::new(
                flashcard_repository,
                flashcard_file_repository,
                flashcard_type_relation_repository,
            );

            let flashcard_type_repository = FlashcardTypeRepository::new(connection.pool.clone());
            let flashcard_type_usecase = FlashcardTypeUseCase::new(flashcard_type_repository);
            let user_usecase = UserUseCase::new(user_repository);
            let identity_user_usecase =
                IdentityUserUseCase::new(identity_password_hasher.clone(), user_usecase.clone());
            let identity_login_usecase = IdentityLoginUseCase::new(
                identity_password_hasher,
                user_usecase.clone(),
                identity_token_helper,
            );
            let app_state = RegularAppState {
                flashcard_usecase,
                flashcard_type_usecase,
                user_usecase,
                identity_user_usecase,
                identity_login_usecase,
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
    ConfigurationHelper::get_value("database.url")
}

fn main() {
    start()
}
