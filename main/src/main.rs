    use axum::{
        http::{HeaderValue, Method}, routing::{get, post}, Router
    };
    use local_ip_address::local_ip;
    use std::sync::Arc;

    mod auth;

    // assuming build_app_state and db functions are here

    use main::{build_app_state, db};
    use tokio::sync::Mutex;
    use tokio_postgres::Client;
    use tower_http::cors::CorsLayer;

    #[tokio::main] 
    async fn main() {
        let redis = build_app_state().await;
        let db = db().await;

    

        let my_local_ip = local_ip().unwrap();
        let location = format!("{}:3000", &my_local_ip);
        let location2=format!("{}:1420",&my_local_ip);
        let listener = tokio::net::TcpListener::bind(&location).await.unwrap();
    let app = app(AppState {
            redis: Arc::new(Mutex::new(redis)),
            db: Arc::new(db),
        },);
        println!("🚀 Server running at http://{}", location);
        axum::serve(listener, app).await.unwrap();
    }

    fn app(state: AppState,) -> Router {

   let cors = CorsLayer::very_permissive();

        Router::new()
            .route("/", get(index))
            .route("/signup", post(auth::handler::sign_up::sign_up))
            .route("/login", post(auth::handler::log_in::log_in))
            .route(
                "/signup/code",
                post(auth::handler::handle_insert::signup_handle_insert::signup_handle_insert),
            )
            .route(
                "/login/code",
                post(auth::handler::handle_insert::login_handle_insert::login_hanle_insert),
            )
            .layer(cors)
            .with_state(Arc::new(state))
    }

    async fn index() -> &'static str {
        "Hello, World!"
    }

    pub struct AppState {
        pub redis: Arc<Mutex<redis::aio::MultiplexedConnection>>,
        pub db: Arc<Client>,
    }
