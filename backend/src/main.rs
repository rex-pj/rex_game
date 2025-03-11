use std::sync::Arc;

use app_state::RegularAppState;
use axum::routing::{patch, put};
use axum::{routing::get, routing::post, Router};
use handlers::flashcard_type_handler::FlashcardTypeHandler;
use handlers::setup_handler::SetupHandler;
use handlers::user_handler::UserHandler;
use handlers::{
    authentication_handler::AuthenticationHandler, flashcard_handler::FlashcardHandler,
};
use middlewares::auth_middleware::{AuthLayer, AuthMiddleware};
use rex_game_application::identities::identity_authenticate_usecase::IdentityAuthenticateUseCase;
use rex_game_application::identities::identity_user_usecase::IdentityUserUseCase;
use rex_game_application::{
    flashcard_types::flashcard_type_usecase::FlashcardTypeUseCase,
    flashcards::flashcard_usecase::FlashcardUseCase, users::user_usecase::UserUseCase,
};
use rex_game_infrastructure::helpers::configuration_helper::ConfigurationHelper;
use rex_game_infrastructure::identities::identity_password_hasher::IdentityPasswordHasher;
use rex_game_infrastructure::identities::identity_token_helper::IdentityTokenHelper;
use rex_game_infrastructure::repositories::role_repository::RoleRepository;
use rex_game_infrastructure::repositories::user_role_repository::UserRoleRepository;
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
use tower::ServiceBuilder;
pub mod app_state;
pub mod handlers;
pub mod helpers;
pub mod middlewares;
pub mod view_models;

fn build_routers(app_state: RegularAppState) -> Router {
    Router::new()
        .route(
            "/flashcards",
            post(FlashcardHandler::create_flashcard::<RegularAppState>),
        )
        .route(
            "/flashcards/{id}",
            patch(FlashcardHandler::update_flashcard::<RegularAppState>),
        )
        .route(
            "/flashcard-types",
            post(FlashcardTypeHandler::create_flashcard_type::<RegularAppState>),
        )
        .route(
            "/flashcard-types/{id}",
            put(FlashcardTypeHandler::update_flashcard_type::<RegularAppState>),
        )
        .route(
            "/auth/refresh",
            post(AuthenticationHandler::refresh_access_token::<RegularAppState>),
        )
        .layer(ServiceBuilder::new().layer(AuthLayer {
            app_state: Arc::new(app_state.clone()),
        }))
        // public routes
        .route(
            "/flashcards",
            get(FlashcardHandler::get_flashcards::<RegularAppState>),
        )
        .route(
            "/flashcards/{id}",
            get(FlashcardHandler::get_flashcard_by_id::<RegularAppState>),
        )
        .route(
            "/flashcards/images/{id}",
            get(FlashcardHandler::get_flashcard_image::<RegularAppState>),
        )
        .route(
            "/flashcard-types",
            get(FlashcardTypeHandler::get_flashcard_types::<RegularAppState>),
        )
        .route(
            "/flashcard-types/{id}",
            get(FlashcardTypeHandler::get_flashcard_type_by_id::<RegularAppState>),
        )
        .route(
            "/auth/login",
            post(AuthenticationHandler::login::<RegularAppState>),
        )
        .route("/users", post(UserHandler::create_user::<RegularAppState>))
        .route("/setup", post(SetupHandler::setup::<RegularAppState>))
        .with_state(app_state)
}

#[tokio::main]
async fn start() {
    let configuration_helper = Arc::new(ConfigurationHelper::new());
    let connection_str = configuration_helper.get_value("database.url");
    let db_connection = match SeaOrmConnection::new(&connection_str).await {
        Ok(connection) => {
            println!("Successfully connected to the database.");
            connection
        }
        Err(err) => return eprintln!("Failed to connect to the database: {:?}", err),
    };

    let flashcard_repository = FlashcardRepository::new(Arc::clone(&db_connection.pool));
    let flashcard_file_repository = FlashcardFileRepository::new(Arc::clone(&db_connection.pool));
    let flashcard_type_relation_repository =
        FlashcardTypeRelationRepository::new(Arc::clone(&db_connection.pool));
    let user_repository = UserRepository::new(Arc::clone(&db_connection.pool));
    let role_repository = RoleRepository::new(Arc::clone(&db_connection.pool));
    let user_role_repository = UserRoleRepository::new(Arc::clone(&db_connection.pool));
    let identity_password_hasher = IdentityPasswordHasher::new();
    let identity_token_helper = IdentityTokenHelper::new(configuration_helper);

    let flashcard_usecase = FlashcardUseCase::new(
        flashcard_repository,
        flashcard_file_repository,
        flashcard_type_relation_repository,
    );

    let flashcard_type_repository = FlashcardTypeRepository::new(Arc::clone(&db_connection.pool));
    let flashcard_type_usecase = FlashcardTypeUseCase::new(flashcard_type_repository);
    let user_usecase = UserUseCase::new(user_repository, role_repository, user_role_repository);
    let identity_user_usecase =
        IdentityUserUseCase::new(identity_password_hasher.clone(), user_usecase.clone());
    let identity_authenticate_usecase = IdentityAuthenticateUseCase::new(
        identity_password_hasher,
        user_usecase.clone(),
        identity_token_helper,
    );
    let app_state = RegularAppState {
        flashcard_usecase,
        flashcard_type_usecase,
        user_usecase,
        identity_user_usecase,
        identity_authenticate_usecase,
    };

    let app = build_routers(app_state);
    let listener = TcpListener::bind("0.0.0.0:3400").await.unwrap();
    println!("The application is running at: http://localhost:3400");
    axum::serve(listener, app).await.unwrap();
}

fn main() {
    start()
}
