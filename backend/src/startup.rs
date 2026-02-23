use crate::app_state;
use crate::middlewares::rate_limit_middleware::{
    api_rate_limiter, auth_rate_limiter, strict_rate_limiter,
};
use crate::routings::app_routing::AppRouting;
use app_state::{AppState, Helpers, RateLimiters, UseCases};
use axum::http::request::Parts;
use axum::http::HeaderValue;
use axum::Router;
use hyper::{header, Method};
// New modular imports
use rex_game_games::{
    FlashcardFileRepository, FlashcardRepository, FlashcardTypeRelationRepository,
    FlashcardTypeRepository, ScoringRepository, ScoringRepositoryTrait, ScoringUseCase,
};
use rex_game_games::{FlashcardTypeUseCase, FlashcardUseCase};
use rex_game_identity::{
    IdentityAuthenticateUseCase, IdentityAuthorizeUseCase, IdentityUserTokenUseCase,
    IdentityUserUseCase, PermissionUseCase, RoleUseCase, UserUseCase,
};
use rex_game_identity::{
    IdentityPasswordHasher, IdentityTokenHelper, PermissionRepository, RolePermissionRepository,
    RoleRepository, UserPermissionRepository, UserRepository, UserRoleRepository,
    UserTokenRepository,
};
use rex_game_mail_templates::application::MailTemplateUseCase;
use rex_game_mail_templates::MailTemplateRepository;
use rex_game_shared::infrastructure::database::SeaOrmConnection;
use rex_game_shared::infrastructure::{
    database::transaction_manager::TransactionManager,
    helpers::{
        configuration_helper::ConfigurationHelper, datetime_helper::DateTimeHelper,
        email_helper::EmailHelper, html_helper,
    },
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[tokio::main]
pub async fn start() {
    // Load environment variables from .env file
    ConfigurationHelper::init();

    // Initialize structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,rex_game=debug,sqlx=warn".into()),
        )
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .json()
        .init();

    tracing::info!("Starting qHortus Backend Server");

    let configuration_helper = Arc::new(ConfigurationHelper::new());
    let connection_str = configuration_helper.get("DATABASE_URL");

    tracing::info!("Connecting to database...");
    let db_connection = match SeaOrmConnection::new(&connection_str).await {
        Ok(connection) => {
            tracing::info!("Successfully connected to database");
            connection
        }
        Err(err) => {
            tracing::error!(error = ?err, "Failed to connect to database");
            return;
        }
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
        user_permission_repository.clone(),
        identity_password_hasher.clone(),
    );
    let role_usecase = RoleUseCase::new(
        role_repository,
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
    let date_time_helper = DateTimeHelper::new();
    let transaction_manager = TransactionManager::new(Arc::clone(&db_connection.pool));
    let email_helper = EmailHelper::new();
    let identity_user_token_usecase = IdentityUserTokenUseCase::new(user_token_repository);
    let mail_template_repository = MailTemplateRepository::new(Arc::clone(&db_connection.pool));
    let mail_template_usecase = MailTemplateUseCase::new(mail_template_repository);
    let html_helper = html_helper::HtmlHelper::new();

    // Scoring module
    let scoring_repository: Arc<dyn ScoringRepositoryTrait> =
        Arc::new(ScoringRepository::new(Arc::clone(&db_connection.pool)));
    let scoring_usecase = ScoringUseCase::new(scoring_repository);

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
        scoring: scoring_usecase,
    };

    // Create helpers group
    let helpers = Helpers {
        email: email_helper,
        date_time: date_time_helper,
        html: html_helper,
        configuration: configuration_helper.clone(),
        token: identity_token_helper,
    };

    // Create rate limiters
    let rate_limiters = RateLimiters {
        auth: auth_rate_limiter(),
        api: api_rate_limiter(),
        strict: strict_rate_limiter(),
    };

    // Create the main application state
    let app_state = AppState {
        usecases,
        helpers,
        db_connection: Arc::clone(&db_connection.pool),
        transaction_manager,
        rate_limiters,
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
                    .clone()
                    .get_array("CORS_ALLOW_ORIGINS")
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

    // Get server configuration from environment
    let config = ConfigurationHelper::new();
    let server_host = config.get_optional("SERVER_HOST");
    let server_host = if server_host.is_empty() {
        "0.0.0.0".to_string()
    } else {
        server_host
    };
    let server_port = config.get_optional("SERVER_PORT");
    let server_port = if server_port.is_empty() {
        "3400".to_string()
    } else {
        server_port
    };
    let bind_addr = format!("{}:{}", server_host, server_port);

    tracing::info!("Binding server to {}", bind_addr);
    let listener = TcpListener::bind(&bind_addr).await.unwrap();
    println!(
        "The application is running at: http://{}:{}",
        server_host, server_port
    );
    tracing::info!("🛡️  Rate limiting enabled: Auth (5/sec), API (30/sec), Password (3/min)");
    tracing::info!("📊 Logging level: INFO (set RUST_LOG env var to change)");

    if let Err(err) = axum::serve(listener, stated_routes).await {
        tracing::error!(error = ?err, "Server error");
    }
}
