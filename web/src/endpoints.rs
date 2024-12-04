use std::sync::Arc;

use veno_core::config::AppConfig;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};

pub async fn routes(config: AppConfig) {
    let shared_config = Arc::new(config);
    let app = Router::new()
        .route("/check", get(check))
        // .route("/users", post(create_user))
        .with_state(shared_config);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn check(State(config): State<Arc<AppConfig>>) -> impl IntoResponse {
    match config.check_artifacts().await {
        Ok(response) => (StatusCode::OK, response),
        Err(e) => {
            println!("Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal Server Error"),
            )
        }
    }
}

// async fn create_user(
//     // this argument tells axum to parse the request body
//     // as JSON into a `CreateUser` type
//     Json(payload): Json<CreateUser>,
// ) -> (StatusCode, Json<User>) {
//     // insert your application logic here
//     let user = User {
//         id: 1337,
//         username: payload.username,
//     };
//
//     // this will be converted into a JSON response
//     // with a status code of `201 Created`
//     (StatusCode::CREATED, Json(user))
// }
//
// // the input to our `create_user` handler
// #[derive(Deserialize)]
// struct CreateUser {
//     username: String,
// }
//
// // the output to our `create_user` handler
// #[derive(Serialize)]
// struct User {
//     id: u64,
//     username: String,
// }
