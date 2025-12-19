use crate::state::AppState;
use crate::roster::model::{Roster, RosterQuery};
use crate::error::AppError;
use sqlx::{MySql, QueryBuilder};

pub struct RosterRepository;

impl RosterRepository {
    pub async fn find_by(state: &AppState, query: RosterQuery) -> Result<Vec<Roster>, AppError> {
        let mut builder: QueryBuilder<MySql> = QueryBuilder::new("SELECT * FROM roster WHERE 1=1");

        if let Some(template_id) = query.template_id {
            builder.push(" AND template_id = ");
            builder.push_bind(template_id);
        }

        if let Some(title) = query.title {
            builder.push(" AND title LIKE ");
            builder.push_bind(format!("%{}%", title));
        }

        if let Some(preview_image_url) = query.preview_image_url {
            builder.push(" AND preview_image_url LIKE ");
            builder.push_bind(format!("%{}%", preview_image_url));
        }

        if let Some(poster_url) = query.poster_url {
            builder.push(" AND poster_url LIKE ");
            builder.push_bind(format!("%{}%", poster_url));
        }

        if let Some(orientation) = query.orientation {
            builder.push(" AND orientation = ");
            builder.push_bind(orientation);
        }

        if let Some(aspect_ratio) = query.aspect_ratio {
            builder.push(" AND aspect_ratio = ");
            builder.push_bind(aspect_ratio);
        }

        if let Some(card_effect) = query.card_effect {
            builder.push(" AND card_effect = ");
            builder.push_bind(card_effect);
        }
        
        if let Some(card_type) = query.card_type {
            builder.push(" AND card_type = ");
            builder.push_bind(card_type);
        }

        if let Some(background) = query.background {
            builder.push(" AND background = ");
            builder.push_bind(background);
        }

        let roster = builder.build_query_as::<Roster>()
            .fetch_all(&state.db)
            .await?;
            
        Ok(roster)
    }
}