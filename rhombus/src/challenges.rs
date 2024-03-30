use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use minijinja::context;

use crate::RhombusRouterState;

pub async fn route_challenges(state: State<RhombusRouterState>) -> impl IntoResponse {
    let challenges = state.db.get_challenges().await;

    Html(
        state
            .jinja
            .get_template("challenges.html")
            .unwrap()
            .render(context! {
                challenges => challenges,
            })
            .unwrap(),
    )
}
