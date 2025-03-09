use axum::{routing::post, Router, extract::Json, response::IntoResponse};
use printpdf::*;
use serde::Deserialize;
use std::io::Cursor;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/generate_pdf", axum::routing::post(generate_pdf));

    axum::Server::bind(&"0.0.0.0:8081".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(serde::Deserialize)]
struct PdfRequest {
    program_name: String,
    activities: Vec<Activity>,
}

#[derive(serde::Deserialize)]
struct Activity {
    time: String,
    number: String,
    responsible: String,
    content: String,
    equipment: String,
    notes: String,
}

async fn generate_pdf(axum::Json(payload): axum::Json<PdfRequest>) -> impl axum::response::IntoResponse {
    // ここでprintpdfを使用してPDFを作成
    // ファイルをバイナリデータとして返す

    axum::response::Response::builder()
        .header("Content-Type", "application/pdf")
        .header("Content-Disposition", "attachment; filename=\"plan.pdf\"")
        .body(axum::body::Body::from(pdf_binary))
        .unwrap()
}
