use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlexWebhookEvent {
    #[serde(rename = "event")]
    pub event: Option<String>,

    #[serde(rename = "user")]
    pub user: Option<bool>,

    #[serde(rename = "owner")]
    pub owner: Option<bool>,

    #[serde(rename = "Account")]
    pub account: Option<Account>,

    #[serde(rename = "Server")]
    pub server: Option<Server>,

    #[serde(rename = "Player")]
    pub player: Option<Player>,

    #[serde(rename = "Metadata")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "thumb")]
    pub thumb: Option<String>,

    #[serde(rename = "title")]
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "librarySectionType")]
    pub library_section_type: Option<String>,

    #[serde(rename = "ratingKey")]
    pub rating_key: Option<String>,

    #[serde(rename = "key")]
    pub key: Option<String>,

    #[serde(rename = "parentRatingKey")]
    pub parent_rating_key: Option<String>,

    #[serde(rename = "grandparentRatingKey")]
    pub grandparent_rating_key: Option<String>,

    #[serde(rename = "guid")]
    pub guid: Option<String>,

    #[serde(rename = "librarySectionID")]
    pub library_section_id: Option<i64>,

    #[serde(rename = "type")]
    pub metadata_type: Option<String>,

    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(rename = "grandparentKey")]
    pub grandparent_key: Option<String>,

    #[serde(rename = "parentKey")]
    pub parent_key: Option<String>,

    #[serde(rename = "grandparentTitle")]
    pub grandparent_title: Option<String>,

    #[serde(rename = "parentTitle")]
    pub parent_title: Option<String>,

    #[serde(rename = "summary")]
    pub summary: Option<String>,

    #[serde(rename = "year")]
    pub year: Option<i64>,

    #[serde(rename = "index")]
    pub index: Option<i64>,

    #[serde(rename = "parentIndex")]
    pub parent_index: Option<i64>,

    #[serde(rename = "ratingCount")]
    pub rating_count: Option<i64>,

    #[serde(rename = "thumb")]
    pub thumb: Option<String>,

    #[serde(rename = "art")]
    pub art: Option<String>,

    #[serde(rename = "parentThumb")]
    pub parent_thumb: Option<String>,

    #[serde(rename = "grandparentThumb")]
    pub grandparent_thumb: Option<String>,

    #[serde(rename = "grandparentArt")]
    pub grandparent_art: Option<String>,

    #[serde(rename = "addedAt")]
    pub added_at: Option<i64>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "local")]
    pub local: Option<bool>,

    #[serde(rename = "publicAddress")]
    pub public_address: Option<String>,

    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(rename = "uuid")]
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    #[serde(rename = "title")]
    pub title: Option<String>,

    #[serde(rename = "uuid")]
    pub uuid: Option<String>,
}

