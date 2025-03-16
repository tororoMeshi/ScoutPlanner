use axum::{
    body::Bytes,
    http::{header, HeaderMap},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(show_form))
        .route("/generate_markdown", post(generate_markdown))
        .route("/export_yaml", post(export_yaml))
        .route("/import_yaml", post(import_yaml));

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn show_form() -> Html<&'static str> {
    Html(include_str!("form.html"))
}

#[derive(Serialize, Deserialize)]
struct FormInput {
    program_name: String,
    location: String,
    meeting_point: String,
    dismissal_point: String,
    overall_responsible: String,
    author: String,
    creation_date: String,
    version: String, // String 型に変更済み
    activity_date: String,
    emergency_contacts: Vec<String>,
    activity_purpose: String,
    activity_objectives: Vec<String>,
    activity_contents: Vec<String>,
    progress: String,
    #[serde(default)]
    time: Vec<String>,
    #[serde(default)]
    responsible: Vec<String>,
    #[serde(default)]
    program_item: Vec<String>,
    #[serde(default)]
    gathering_method: Vec<String>,
    #[serde(default)]
    items: Vec<String>,
    #[serde(default)]
    participants: Vec<String>,
    budget: String,
    rain_plan: String,
    footer_equipment: String,
    #[serde(default)]
    products: Vec<String>,
    #[serde(default)]
    memo: Vec<String>,
    // 小項目：各活動グループ内のテーブルの各行（4要素）の配列をまとめたもの
    #[serde(default)]
    sub_items: Vec<Vec<Vec<String>>>,
}

async fn generate_markdown(body: Bytes) -> impl IntoResponse {
    let input: FormInput = match serde_yaml::from_slice(&body) {
        Ok(data) => data,
        Err(err) => {
            let mut error_headers = HeaderMap::new();
            error_headers.insert(
                header::CONTENT_TYPE,
                "text/plain; charset=utf-8".parse().unwrap(),
            );
            return (error_headers, format!("YAMLパースエラー: {}", err));
        }
    };

    let activity_details = input
        .time
        .iter()
        .enumerate()
        .map(|(i, _)| {
            format!(
                "### {}. {}\n時間: {} 担当: {} 集合形態: {}\n",
                i + 1,
                input.program_item.get(i).unwrap_or(&String::new()),
                input.time.get(i).unwrap_or(&String::new()),
                input.responsible.get(i).unwrap_or(&String::new()),
                input.gathering_method.get(i).unwrap_or(&String::new())
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let markdown = format!(
        r#"# ScoutPlanner 計画書

## ヘッダー情報
- プログラム名称: {}
- 活動場所: {}
- 集合場所: {}
- 解散場所: {}
- 活動日: {}
- プログラム全体の担当者: {}
- 作成者: {}
- 作成日: {}
- 作成バージョン: {}
- 緊急連絡先:
{}

## 目的・内容
- 活動目的: {}
- 活動目標:
{}
- 活動内容:
{}
- 進歩: {}

## 活動詳細
{}

## フッター情報
- 持ち物:
{}
- 参加者:
{}
- 予算: {}
- 雨天時: {}
- 備品: {}
- 制作物:
{}
- メモ:
{}
"#,
        input.program_name,
        input.location,
        input.meeting_point,
        input.dismissal_point,
        input.activity_date,
        input.overall_responsible,
        input.author,
        input.creation_date,
        input.version,
        input.emergency_contacts.join("\n- "),
        input.activity_purpose,
        input.activity_objectives.join("\n- "),
        input.activity_contents.join("\n- "),
        input.progress,
        activity_details,
        input.items.join("\n- "),
        input.participants.join("\n- "),
        input.budget,
        input.rain_plan,
        input.footer_equipment,
        input.products.join("\n- "),
        input.memo.join("\n- ")
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "text/markdown; charset=utf-8".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        "attachment; filename=\"scoutplanner.md\"".parse().unwrap(),
    );

    (headers, markdown)
}

async fn export_yaml(body: Bytes) -> impl IntoResponse {
    // パースに成功したデータ（sub_items も含む）をそのまま YAML 文字列に変換
    let input: FormInput = match serde_yaml::from_slice(&body) {
        Ok(data) => data,
        Err(err) => {
            let mut error_headers = HeaderMap::new();
            error_headers.insert(
                header::CONTENT_TYPE,
                "text/plain; charset=utf-8".parse().unwrap(),
            );
            return (error_headers, format!("YAMLパースエラー: {}", err));
        }
    };

    let yaml = match serde_yaml::to_string(&input) {
        Ok(y) => y,
        Err(err) => {
            let mut error_headers = HeaderMap::new();
            error_headers.insert(
                header::CONTENT_TYPE,
                "text/plain; charset=utf-8".parse().unwrap(),
            );
            return (error_headers, format!("YAML生成エラー: {}", err));
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/x-yaml; charset=utf-8".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        "attachment; filename=\"scoutplanner.yaml\""
            .parse()
            .unwrap(),
    );

    (headers, yaml)
}

async fn import_yaml(body: Bytes) -> impl IntoResponse {
    let input: FormInput = match serde_yaml::from_slice(&body) {
        Ok(data) => data,
        Err(err) => return Html(format!("YAMLパースエラー: {}", err)),
    };

    let html_content = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>インポートされた計画書</title>
</head>
<body>
    <h1>インポートされた計画書</h1>
    <pre>{}</pre>
</body>
</html>"#,
        serde_yaml::to_string(&input).unwrap()
    );

    Html(html_content)
}
