use axum::{
    extract::Form,
    http::header,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(show_form))
        .route("/generate_pdf", post(submit_form));

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn show_form() -> impl IntoResponse {
    Html(include_str!("form.html"))
}

#[derive(Serialize, Deserialize)]
struct FormInput {
    program_name: String,
    location: String,
    meeting_point: String,
    objective: String,
    progress: String,
    number: Vec<usize>,
    time: Vec<String>,
    responsible: Vec<String>,
    program_item: Vec<String>,
    gathering_method: Vec<String>,
    item_details: Vec<String>,
    item_comments: Vec<String>,
    equipment: Vec<String>,
    notes: Vec<String>,
    items: String,
    participants: String,
    budget: String,
    rain_plan: String,
    footer_equipment: String,
    products: String,
    memo: String,
}

async fn submit_form(Form(input): Form<FormInput>) -> impl IntoResponse {
    let client = reqwest::Client::new();
    let backend_url = std::env::var("PDF_BACKEND_URL").unwrap_or("http://pdf-backend:8081".into());

    let response = client
        .post(format!("{}/generate_pdf", backend_url))
        .json(&input)
        .send()
        .await
        .unwrap();

    let pdf_bytes = response.bytes().await.unwrap();

    (
        [
            (header::CONTENT_TYPE, "application/pdf"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"scoutplanner.pdf\"",
            ),
        ],
        pdf_bytes.to_vec(),
    )
}
