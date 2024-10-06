mod slackbot;
use axum::{
    routing::{get,post},
    Router,
    http::StatusCode,
};
use crate::slackbot::slackbot;
use dotenv::dotenv;
use tracing_subscriber::layer::SubscriberExt;

#[tokio::main]
async fn main(){
    init().await;

    //build our application with a single route
    let app = Router::new().route("/", get(handler))
                            .route("/slackbot", post(slackbot));

    //run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> (StatusCode, String){
    (StatusCode::OK, String::from("Hello, World"))
}

async fn init(){
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_level(true)
        .with_line_number(true)
    );
    dotenv().ok();
}
