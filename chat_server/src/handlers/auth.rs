use axum::response::IntoResponse;

pub(crate) async fn sigin_handler() -> impl IntoResponse {
    "signin"
}

pub(crate) async fn signup_handler() -> impl IntoResponse {
    "signup"
}
