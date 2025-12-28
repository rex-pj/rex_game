use crate::app_state;
use crate::routings::app_routing::AppRouting;
use app_state::{AppState, Helpers, UseCases};
use axum::http::request::Parts;
use axum::http::HeaderValue;
use axum::Router;
use hyper::{header, Method};
// New modular imports
use rex_game_games::{
    FlashcardFileRepository, FlashcardRepository, FlashcardTypeRelationRepository,
    FlashcardTypeRepository, FlashcardTypeUseCase, FlashcardUseCase,
};
use rex_game_identity::{
    IdentityAuthenticateUseCase, IdentityAuthorizeUseCase, IdentityPasswordHasher,
    IdentityTokenHelper, IdentityUserTokenUseCase, IdentityUserUseCase, PermissionRepository,
    PermissionUseCase, RolePermissionRepository, RoleRepository, RoleUseCase,
    UserPermissionRepository, UserRepository, UserRoleRepository, UserTokenRepository, UserUseCase,
};
use rex_game_mail_templates::{MailTemplateRepository, MailTemplateUseCase};
use rex_game_shared_kernel::infrastructure::database::SeaOrmConnection;
use rex_game_shared_kernel::infrastructure::{
    database::transaction_manager::TransactionManager,
    helpers::{
        configuration_helper::ConfigurationHelper, datetime_helper::DateTimeHelper,
        email_helper::EmailHelper, file_helper::FileHelper, html_helper,
    },
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[tokio::main]
pub async fn start() {
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
    let permission_repository = PermissionRepository::new(Arc::clone(&db_connection.pool));
    let user_permission_repository = UserPermissionRepository::new(Arc::clone(&db_connection.pool));
    let role_permission_repository = RolePermissionRepository::new(Arc::clone(&db_connection.pool));
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
        permission_repository.clone(),
        user_permission_repository.clone(),
        identity_password_hasher.clone(),
    );
    let role_usecase = RoleUseCase::new(
        role_repository,
        permission_repository.clone(),
        role_permission_repository.clone(),
        user_role_repository.clone(),
    );
    let permission_usecase = PermissionUseCase::new(permission_repository);
    let user_token_repository = UserTokenRepository::new(Arc::clone(&db_connection.pool));
    let identity_user_usecase = IdentityUserUseCase::new(
        identity_password_hasher.clone(),
        user_usecase.clone(),
        role_usecase.clone(),
        identity_token_helper.clone(),
    );
    let identity_authenticate_usecase = IdentityAuthenticateUseCase::new(
        configuration_helper.clone(),
        identity_password_hasher,
        user_usecase.clone(),
        identity_token_helper.clone(),
    );
    let identity_authorize_usecase = IdentityAuthorizeUseCase::new(
        user_role_repository,
        user_permission_repository,
        role_permission_repository,
    );
    let file_helper = FileHelper::new();
    let date_time_helper = DateTimeHelper::new();
    let transaction_manager = TransactionManager::new(Arc::clone(&db_connection.pool));
    let email_helper = EmailHelper::new();
    let identity_user_token_usecase = IdentityUserTokenUseCase::new(user_token_repository);
    let mail_template_repository = MailTemplateRepository::new(Arc::clone(&db_connection.pool));
    let mail_template_usecase = MailTemplateUseCase::new(mail_template_repository);
    let html_helper = html_helper::HtmlHelper::new();

    // Create use cases group
    let usecases = UseCases {
        flashcard: flashcard_usecase,
        flashcard_type: flashcard_type_usecase,
        user: user_usecase,
        identity_user: identity_user_usecase,
        identity_authenticate: identity_authenticate_usecase,
        role: role_usecase,
        permission: permission_usecase,
        identity_authorize: identity_authorize_usecase,
        identity_user_token: identity_user_token_usecase,
        mail_template: mail_template_usecase,
    };

    // Create helpers group
    let helpers = Helpers {
        file: file_helper,
        email: email_helper,
        date_time: date_time_helper,
        html: html_helper,
        configuration: configuration_helper.clone(),
        token: identity_token_helper,
    };

    // Create the main application state
    let app_state = AppState {
        usecases,
        helpers,
        db_connection: Arc::clone(&db_connection.pool),
        transaction_manager,
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
