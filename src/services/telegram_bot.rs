use std::{collections::HashMap, time::Duration};

#[derive(Clone)]
pub struct TelegramBotService {
    chat_id: String,
    token: String,
    endpoint: String,
    client: reqwest::Client,
}

impl TelegramBotService {
    /// Create the service to handle request to TelegramAPI
    pub fn new(chat_id: String, token: String) -> Self {
        // TODO: Make this endpoint injectable for testing using mock API
        let endpoint = "https://api.telegram.org".to_owned();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();
        Self {
            chat_id,
            token,
            endpoint,
            client,
        }
    }

    /// Make the TelegramAPI url with endpoint and token of the service
    ///
    /// ```
    /// make_url("sendMessage"); // https://api.telegram.org/bot<token>/sendMessage
    /// ```
    fn make_url(&self, path: String) -> String {
        format!("{}/bot{}{path}", self.endpoint, self.token)
    }

    /// Send a message to chat room with `chat_id` and `bot_token` given in the service object
    pub async fn send_message(&self, msg: String) -> Result<(), String> {
        let url = self.make_url("/sendMessage".to_string());
        let mut map = HashMap::new();
        map.insert("chat_id", self.chat_id.clone());
        map.insert("text", msg);

        match self.client.post(url).json(&map).send().await {
            Err(err) => {
                tracing::error!("Error request telegram: {:?}", err);
                Err("cannot request telegram".to_string())
            }
            Ok(res) => {
                tracing::debug!("Telegram resp: {:?}", res);
                Ok(())
            }
        }
    }
}
