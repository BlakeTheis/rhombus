use axum::{
    extract::{Path, State},
    http::{Response, Uri},
    response::{Html, IntoResponse},
    Extension, Form,
};
use minijinja::context;
use serde::Deserialize;

use super::{auth::User, locales::Languages, router::RouterState};

pub async fn route_challenges(
    state: State<RouterState>,
    Extension(user): Extension<User>,
    Extension(lang): Extension<Languages>,
    uri: Uri,
) -> impl IntoResponse {
    let challenge_data = state.db.get_challenges();
    let team = state.db.get_team_from_id(user.team_id);
    let (challenge_data, team) = tokio::join!(challenge_data, team);
    let challenge_data = challenge_data.unwrap();
    let team = team.unwrap();

    Html(
        state
            .jinja
            .get_template("challenges.html")
            .unwrap()
            .render(context! {
                lang => lang,
                user => user,
                uri => uri.to_string(),
                challenges => challenge_data.challenges,
                categories => challenge_data.categories,
                authors => challenge_data.authors,
                team => team,
            })
            .unwrap(),
    )
}

pub async fn route_challenge_view(
    state: State<RouterState>,
    Extension(user): Extension<User>,
    Extension(lang): Extension<Languages>,
    challenge_id: Path<i64>,
    uri: Uri,
) -> impl IntoResponse {
    let challenge_data = state.db.get_challenges().await.unwrap();
    let challenge = challenge_data
        .challenges
        .iter()
        .find(|c| challenge_id.eq(&c.id))
        .unwrap();
    let category = challenge_data
        .categories
        .iter()
        .find(|c| challenge.category_id.eq(&c.id))
        .unwrap();

    Html(
        state
            .jinja
            .get_template("challenge.html")
            .unwrap()
            .render(context! {
                lang => lang,
                user => user,
                uri => uri.to_string(),
                challenge => challenge,
                category => category,
            })
            .unwrap(),
    )
}

#[derive(Deserialize)]
pub struct SubmitChallenge {
    flag: String,
}

pub async fn route_challenge_submit(
    state: State<RouterState>,
    Extension(user): Extension<User>,
    Extension(lang): Extension<Languages>,
    challenge_id: Path<i64>,
    uri: Uri,
    Form(form): Form<SubmitChallenge>,
) -> impl IntoResponse {
    let challenge_data = state.db.get_challenges().await.unwrap();
    let challenge = challenge_data
        .challenges
        .iter()
        .find(|c| challenge_id.eq(&c.id))
        .unwrap();

    if challenge.flag != form.flag {
        let html = state
            .jinja
            .get_template("challenge-submit.html")
            .unwrap()
            .render(context! {
                lang => lang,
                user => user,
                uri => uri.to_string(),
                error => "Incorrect flag",
            })
            .unwrap();
        return Response::builder()
            .header("content-type", "text/html")
            .body(html)
            .unwrap();
    }

    if state
        .db
        .solve_challenge(user.id, user.team_id, challenge.id)
        .await
        .is_err()
    {
        let html = state
            .jinja
            .get_template("challenge-submit.html")
            .unwrap()
            .render(context! {
                lang => lang,
                user => user,
                uri => uri.to_string(),
                error => "Unknown database error",
            })
            .unwrap();
        return Response::builder()
            .header("content-type", "text/html")
            .body(html)
            .unwrap();
    }

    let html = state
        .jinja
        .get_template("challenge-submit.html")
        .unwrap()
        .render(context! {
            lang => lang,
            user => user,
            uri => uri.to_string(),
        })
        .unwrap();

    Response::builder()
        .header("content-type", "text/html")
        .header("hx-trigger", "{\"manualRefresh\":true,\"closeModal\":true}")
        .body(html)
        .unwrap()
}
