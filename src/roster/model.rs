use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, FromRow)]
pub struct Roster {
    #[schema(example = 1)]
    pub id: i64,
    #[schema(example = "template_001")]
    pub template_id: String,
    #[schema(example = "PSA评级卡")]
    pub title: Option<String>,
    #[schema(example = "https://example.com/preview.mp4")]
    pub preview_image_url: Option<String>,
    #[schema(example = "https://example.com/poster.png")]
    pub poster_url: Option<String>,
    #[schema(example = "竖")]
    pub orientation: Option<String>,
    #[schema(example = "PSA评级卡")]
    pub card_type: Option<String>,
    #[schema(example = "实物切割")]
    pub card_effect: Option<String>,
    #[schema(example = "灵卡岩石")]
    pub background: Option<String>,
    #[schema(example = "3:4")]
    pub aspect_ratio: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct RosterQuery {
    pub template_id: Option<String>,
    pub title: Option<String>,
    pub preview_image_url: Option<String>,
    pub poster_url: Option<String>,
    pub orientation: Option<String>,
    pub card_type: Option<String>,
    pub card_effect: Option<String>,
    pub background: Option<String>,
    pub aspect_ratio: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateRosterRequest {
    #[schema(example = "template_001")]
    pub template_id: String,
    #[schema(example = "PSA评级卡")]
    pub title: Option<String>,
    #[schema(example = "https://example.com/preview.mp4")]
    pub preview_image_url: Option<String>,
    #[schema(example = "https://example.com/poster.png")]
    pub poster_url: Option<String>,
    #[schema(example = "https://example.com/1.glb")]
    pub model_url: Option<String>,
    #[schema(example = "竖")]
    pub orientation: Option<String>,
    #[schema(example = "PSA评级卡")]
    pub card_type: Option<String>,
    #[schema(example = "实物切割")]
    pub card_effect: Option<String>,
    #[schema(example = "灵卡岩石")]
    pub background: Option<String>,
    #[schema(example = "3:4")]
    pub aspect_ratio: Option<String>,
}
