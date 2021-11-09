mod slack;

use axum::{
    http::{Response, StatusCode},
    routing::{get, post},
    AddExtensionLayer, Router,
};
// use dotenv::dotenv;
use slack::{handle_slack_commands_api, handle_slack_events_api, handle_slack_interaction_api};

use serde_json::{from_value, json};
use slack_morphism::{SlackApiToken, SlackClient, SlackClientSession};
use slack_morphism_hyper::SlackClientHyperConnector;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    println!("Starting server..");
    setup_tracing();

    // SETUP SHARED SLACK CLIENT
    let slack_bot_token = SlackApiToken::new(
        env::var("SLACK_BOT_TOKEN")
            .unwrap_or_else(|_| "<no_token_provided>".to_string())
            .into(),
    );
    let slack_client = SlackClient::new(SlackClientHyperConnector::new());

    // let slack_workaround = Arc::new(SlackStateWorkaround {
    //     bot_token: slack_bot_token,
    //     slack_client,
    // });
    let slack_shared_session = Arc::new(slack_client.open_session(&slack_bot_token));

    // consolidate slack routes into a separate Router so we can apply slack verification middleware
    let slack_api_router = Router::new()
        .route("/events", post(handle_slack_events_api))
        .route("/interaction", post(handle_slack_interaction_api))
        .route("/commands", post(handle_slack_commands_api));

    let app = Router::new()
        .nest("/slack", slack_api_router)
        .route("/", get(|| async { "Hello, World!" }))
        .layer(TraceLayer::new_for_http())
        // .layer(AddExtensionLayer::new(slack_workaround));
        .layer(AddExtensionLayer::new(slack_shared_session));

    let host_address = env::var("HOST_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    tracing::debug!("listening on {}", &host_address);

    // run it with hyper
    axum::Server::bind(&host_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// until i can get the Session itself working & stored in an arc, will have to rebuild the session every time
pub struct SlackStateWorkaround {
    slack_client: SlackClient<SlackClientHyperConnector>,
    bot_token: SlackApiToken,
}

impl SlackStateWorkaround {
    fn open_session(&self) -> SlackClientSession<SlackClientHyperConnector> {
        self.slack_client.open_session(&self.bot_token)
    }
}

fn setup_tracing() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "example_tracing_aka_logging=debug,tower_http=debug",
        )
    }
    tracing_subscriber::fmt::init();
}
