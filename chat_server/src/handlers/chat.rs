use axum::response::IntoResponse;

pub(crate) async fn list_chat_handler() -> impl IntoResponse {
    "list_chat"
}

pub(crate) async fn create_chat_handler() -> impl IntoResponse {
    "create_chat"
}

pub(crate) async fn updata_chat_handler() -> impl IntoResponse {
    "updata_chat"
}

pub(crate) async fn delete_chat_handler() -> impl IntoResponse {
    "delete_chat"
}
