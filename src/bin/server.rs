use axum::routing::{get, post};
use axum::{Router, Server};
use hello_axum::handler::check::{must_login, no_login, optional_login};
use hello_axum::handler::user::{login, register};
use hello_axum::{hello, not_found_handler, not_implemented_handler, read_file_handler};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/hello", get(hello))
        .nest(
            "/error",
            Router::new()
                .route("/not_implemented", get(not_implemented_handler))
                .route("/read_file", get(read_file_handler)),
        )
        .nest(
            "/user",
            Router::new()
                .route("/register", post(register))
                .route("/login", post(login)),
        )
        .nest(
            "/check",
            Router::new()
                .route("/must_login", get(must_login))
                .route("/optional_login", get(optional_login))
                .route("/no_login", get(no_login)),
        )
        .fallback(not_found_handler);

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
