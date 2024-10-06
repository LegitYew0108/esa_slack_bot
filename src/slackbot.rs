use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::env;
use tracing::{instrument, field::Empty};

#[derive(Debug, Deserialize)]
pub struct Request{
    token: String,
    challenge: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Response{
    ok: bool,
    challenge: Option<String>,
}

#[instrument(skip_all, name="slackbot", fields(name=Empty), level="info")]
pub async fn slackbot(Json(req): Json<Request>) -> (StatusCode, Json<Response>){
    //validate
    let verification_token =
        env::var("VERIFICATION_TOKEN").expect("VERIFICATION_TOKEN must be set");
    if req.token != verification_token{
        tracing::warn!("AuthenticationFailed, token: {}", req.token);
        let res = Json(Response{
            ok: false,
            challenge: None,
        });
        return (StatusCode::BAD_REQUEST, res);
    }

    let res = Json(Response{
        ok: true,
        challenge: req.challenge,
    });

    (StatusCode::OK, res)
}
