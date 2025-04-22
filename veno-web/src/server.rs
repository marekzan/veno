use std::sync::Arc;

use anyhow::Result;
use tokio::signal::{
    self,
    unix::{self, SignalKind},
};
use tracing::info;

use crate::{resources::serve_api, App};

pub async fn start(app: Arc<App>) -> Result<()> {
    info!("Starting server...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, serve_api(app).into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    info!("server shutdown successfully.");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        unix::signal(SignalKind::hangup())
            .expect("failed to install unix signal handler hangup")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("received termination signal, shutting down...");
}
