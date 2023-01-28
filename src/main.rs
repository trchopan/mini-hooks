use axum::{routing::post, Router};
use clap::Parser;
use env_logger::Env;
use mini_hooks::{
    applications::plex_webhook::plex_webhook, services::telegram_bot::TelegramBotService,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The serving addr for the server
    #[clap(long, value_parser, default_value = "127.0.0.1:5500")]
    addr: String,

    /// Telegram chat_id
    #[clap(long, value_parser)]
    chat_id: String,

    /// Telegram bot token
    #[clap(long, value_parser)]
    bot_token: String,
}

#[tokio::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let args = Args::parse();
    let telegram_svc = TelegramBotService::new(args.chat_id, args.bot_token);

    let app = Router::new()
        .route("/plex", post(plex_webhook))
        .with_state(telegram_svc);

    let addr = &args.addr.parse().expect("Cannot parse the addr");
    tracing::info!("listening on {}", addr);
    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
