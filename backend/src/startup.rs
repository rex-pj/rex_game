use crate::app_state;
use crate::routings::app_routing::AppRouting;
use app_state::RegularAppState;
use axum::http::request::Parts;
use axum::http::HeaderValue;
use axum::Router;
use hyper::{header, Method};
use rex_game_application::identities::identity_authenticate_usecase::IdentityAuthenticateUseCase;
use rex_game_application::identities::identity_authorize_usecase::IdentityAuthorizeUseCase;
use rex_game_application::identities::identity_user_usecase::IdentityUserUseCase;
use rex_game_application::roles::role_usecase::RoleUseCase;
use rex_game_application::{
    flashcard_types::flashcard_type_usecase::FlashcardTypeUseCase,
    flashcards::flashcard_usecase::FlashcardUseCase, users::user_usecase::UserUseCase,
};
use rex_game_infrastructure::helpers::configuration_helper::ConfigurationHelper;
use rex_game_infrastructure::helpers::datetime_helper::DateTimeHelper;
use rex_game_infrastructure::helpers::file_helper::FileHelper;
use rex_game_infrastructure::identities::identity_password_hasher::IdentityPasswordHasher;
use rex_game_infrastructure::identities::identity_token_helper::IdentityTokenHelper;
use rex_game_infrastructure::repositories::role_repository::RoleRepository;
use rex_game_infrastructure::repositories::user_role_repository::UserRoleRepository;
use rex_game_infrastructure::transaction_manager::TransactionManager;
use rex_game_infrastructure::{
    repositories::{
        flashcard_file_repository::FlashcardFileRepository,
        flashcard_repository::FlashcardRepository,
        flashcard_type_relation_repository::FlashcardTypeRelationRepository,
        flashcard_type_repository::FlashcardTypeRepository, user_repository::UserRepository,
    },
    seaorm_connection::SeaOrmConnection,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[tokio::main]
pub async fn start() {
    let configuration_helper = Arc::new(ConfigurationHelper::new());
    let connection_str = configuration_helper.clone().get_value("database.url");
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
    let identity_token_helper = IdentityTokenHelper::new(configuration_helper.clone());

    let flashcard_usecase = FlashcardUseCase::new(
        flashcard_repository,
        flashcard_file_repository,
        flashcard_type_relation_repository,
    );

    let flashcard_type_repository = FlashcardTypeRepository::new(Arc::clone(&db_connection.pool));
    let flashcard_type_usecase = FlashcardTypeUseCase::new(flashcard_type_repository);
    let user_usecase = UserUseCase::new(
        user_repository,
        role_repository.clone(),
        user_role_repository.clone(),
    );
    let role_usecase = RoleUseCase::new(role_repository);
    let identity_user_usecase = IdentityUserUseCase::new(
        identity_password_hasher.clone(),
        user_usecase.clone(),
        identity_token_helper.clone(),
    );
    let identity_authenticate_usecase = IdentityAuthenticateUseCase::new(
        identity_password_hasher,
        user_usecase.clone(),
        identity_token_helper,
    );
    let identity_authorize_usecase = IdentityAuthorizeUseCase::new(user_role_repository.clone());
    let file_helper = FileHelper::new();
    let date_time_helper = DateTimeHelper::new();
    let transaction_manager = TransactionManager::new(Arc::clone(&db_connection.pool));
    let app_state = RegularAppState {
        transaction_manager,
        flashcard_usecase,
        flashcard_type_usecase,
        user_usecase,
        identity_user_usecase,
        identity_authenticate_usecase,
        file_helper,
        date_time_helper,
        db_connection: Arc::clone(&db_connection.pool),
        role_usecase: role_usecase,
        identity_authorize_usecase: identity_authorize_usecase,
    };

    let authenticated_routes = AppRouting {
        app_state: Arc::new(app_state.clone()),
    }
    .build_authenticated_routes(Router::new());

    let admin_authenticated_routes = AppRouting {
        app_state: Arc::new(app_state.clone()),
    }
    .build_admin_routes(authenticated_routes);

    let public_routes = AppRouting {
        app_state: Arc::new(app_state.clone()),
    }
    .build_public_routes(admin_authenticated_routes);

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ])
        // allow requests from any origin
        .allow_origin(AllowOrigin::predicate(
            move |origin: &HeaderValue, _parts: &Parts| {
                // fetch list of origins that are allowed for this path
                let allow_origins = configuration_helper
                    .get_array("cors.allow_origin")
                    .into_iter()
                    .map(|f| HeaderValue::from_str(&f))
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap_or_default();
                allow_origins.contains(origin)
            },
        ))
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);
    let app_routes = Router::new().nest("/api", public_routes).layer(cors);
    let stated_routes = app_routes.with_state(app_state);
    let listener = TcpListener::bind("0.0.0.0:3400").await.unwrap();
    println!("The application is running at: http://localhost:3400");
    axum::serve(listener, stated_routes).await.unwrap();
}
