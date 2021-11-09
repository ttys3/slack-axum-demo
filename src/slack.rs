use std::sync::Arc;

use axum::{extract::Extension, extract::Form, http::StatusCode, response::IntoResponse, Json};
// use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use slack_morphism::{
    prelude::{
        SlackCommandEvent, SlackEventCallbackBody, SlackInteractionEvent, SlackPushEvent,
        SlackPushEventCallback,
    },
    ClientResult,
};
use slack_morphism_models::{SlackChannelId, SlackTs};
use tokio::sync::RwLock;
use tracing::{debug, error};

pub async fn handle_slack_events_api(
    // Extension(cached_config_state): Extension<Arc<RwLock<CachedConfigurations>>>,
    // Extension(slack_state): Extension<Arc<StupidSlackStateWorkaround>>,
    Json(payload): Json<SlackPushEvent>,
) -> impl IntoResponse {
    match payload {
        SlackPushEvent::EventCallback(event_req) => {
            // TODO: process events
            (StatusCode::OK, Json(to_value("").unwrap()))
        }
        SlackPushEvent::UrlVerification(url_verify_req) => {
            (StatusCode::OK, Json(to_value(url_verify_req).unwrap()))
        }
        SlackPushEvent::AppRateLimited(rate_limit_req) => {
            // TODO: handle rate limits
            (StatusCode::OK, Json(to_value(rate_limit_req).unwrap()))
        }
    }
}

/// slash commands
pub async fn handle_slack_commands_api(
    // Extension(cached_config_state): Extension<Arc<RwLock<CachedConfigurations>>>,
    Form(payload): Form<SlackCommandEvent>,
) -> impl IntoResponse {
    debug!("SlackCommandEvent");
    (StatusCode::OK, Json(to_value(payload.channel_id).unwrap()))
}

pub async fn handle_slack_interaction_api(
    Json(payload): Json<SlackInteractionEvent>,
) -> impl IntoResponse {
    debug!("Interaction event");
    match payload {
        // TODO
        SlackInteractionEvent::BlockActions(block_actions_event) => {
            (StatusCode::OK, Json(to_value(block_actions_event).unwrap()))
        }
        // TODO
        SlackInteractionEvent::DialogSubmission(dialog_submission_event) => (
            StatusCode::OK,
            Json(to_value(dialog_submission_event).unwrap()),
        ),
        // TODO
        SlackInteractionEvent::MessageAction(message_action_event) => (
            StatusCode::OK,
            Json(to_value(message_action_event).unwrap()),
        ),
        // TODO
        SlackInteractionEvent::Shortcut(shortcut_event) => {
            (StatusCode::OK, Json(to_value(shortcut_event).unwrap()))
        }
        // TODO
        SlackInteractionEvent::ViewClosed(view_closed_event) => {
            (StatusCode::OK, Json(to_value(view_closed_event).unwrap()))
        }
        // TODO
        SlackInteractionEvent::ViewSubmission(view_submission_event) => (
            StatusCode::OK,
            Json(to_value(view_submission_event).unwrap()),
        ),
    }
}
