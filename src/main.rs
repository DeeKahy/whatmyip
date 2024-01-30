use axum::{extract::ConnectInfo, response::Html, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = "0.0.0.0:4305".parse::<SocketAddr>().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Html<String> {
    Html(format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Your IP Address</title>
            <style>
                body {{
                    font-family: 'Arial', sans-serif;
                    background-color: #f0f0f0;
                    text-align: center;
                    padding-top: 50px;
                }}
                h1 {{
                    color: #333;
                }}
                .ip-address {{
                    background-color: #fff;
                    border: 1px solid #ddd;
                    padding: 10px;
                    border-radius: 5px;
                    display: inline-block;
                    margin-top: 10px;
                }}
            </style>
        </head>
        <body>
            <h1>Welcome to the IP Address Checker!</h1>
            <div class="ip-address">Your IP Address is: <strong>{}</strong></div>
        </body>
        </html>
        "#,
        addr
    ))
}
