use std::{collections::HashMap, sync::Arc};

use neveno_core::{config::AppConfig, sink::SinkNotifier, source::checker};

use axum::{extract::State, routing::get, Router};

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
async fn check(State(config): State<Arc<AppConfig>>) -> String {
    let notifiers: HashMap<String, Box<dyn SinkNotifier<Output = String>>> = config
        .notifiers
        .iter()
        .map(|notifier| (notifier.name.clone(), notifier.sink.to_notifier()))
        .collect();

    let mut result = String::new();

    for artifact in &config.artifacts {
        let Ok(Some(latest_version)) = checker::check(artifact).await else {
            return "Error checking for updates".to_string();
        };

        for notifier_name in &artifact.notifier {
            if let Some(notifier) = notifiers.get(notifier_name) {
                println!("Sending notification to {}", notifier_name);
                match notifier.send(&latest_version).await {
                    Ok(_) => result.push_str(&format!("Sent notification to {}\n", notifier_name)),
                    Err(e) => {
                        result.push_str(format!("Error sending notification: {}", e).as_str());
                    }
                }
            }
        }
    }
    result
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
