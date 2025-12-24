use axum::extract::{State, Query};
use crate::{
    error::AppError,
    state::AppState,
    roster::{model::{Roster, RosterQuery}, RosterRepository},
    response::ApiResponse,
};

/// Get all rosters
#[utoipa::path(
    get,
    path = "/api/v1/roster",
    tag = "Roster",
    params(
        RosterQuery
    ),
    responses(
        (status = 200, description = "List roster", body = ApiResponse<Vec<Roster>>)
    )
)]
pub async fn list_roster(
    State(state): State<AppState>,
    Query(query): Query<RosterQuery>,
) -> Result<ApiResponse<Vec<Roster>>, AppError> {
    let roster = RosterRepository::find_by(&state, query).await?;
    Ok(ApiResponse::success_list(roster))
}