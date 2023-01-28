use crate::{
    models::plex_webhook_event::PlexWebhookEvent, services::telegram_bot::TelegramBotService,
};
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};

pub async fn plex_webhook(
    State(telegram_svc): State<TelegramBotService>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, StatusCode> {
    let mut payload: Option<PlexWebhookEvent> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.text().await.unwrap();
        payload = serde_json::from_str(&data).unwrap_or(None);
        if payload.is_some() {
            // Only take the first event payload
            break;
        }
    }

    let Some(event) = payload else { return Err(StatusCode::BAD_REQUEST); };

    let event_name = event.event.unwrap_or("unknown-event".to_owned());

    if event_name != "media.play" {
        tracing::info!("Nothing to handle for event: {}", event_name);
        return Ok(StatusCode::OK);
    };

    // Event is "media.play"

    let meta = event.metadata.unwrap();
    tracing::debug!("Payload {:?}", meta);
    let unknown = || "<unknown>".to_string();

    let msg = format!(
        "Playing: {title} {year}\nType: {type}\nSummary: {summary}",
        title=meta.title.unwrap_or(unknown()),
        year=meta.year.map(|y| format!("({y})")).unwrap_or(unknown()),
        type=meta.metadata_type.unwrap_or(unknown()),
        summary=meta.summary.unwrap_or(unknown()),
    );

    let _ = telegram_svc.send_message(msg).await;

    Ok(StatusCode::OK)
}
