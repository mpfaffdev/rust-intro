use axum::extract::Path;
use axum::Router;
use axum::routing::get;
use rust_intro::rust_intro::DataMessage;

pub(crate) fn init_routes() -> Router {
    Router::new()
        .route("/", get(route_root))
        .route("/test", get(route_test))
        .route("/test/:path_param", get(route_test_with_param))
        .route("/proto/serialize/:path_param", get(route_serialize_testparam))
}

async fn route_root() -> &'static str {
    "Welcome to rust-intro"
}

async fn route_test() -> &'static str {
    "Test route"
}

async fn route_test_with_param(Path(path_param): Path<String>) -> String {
    format!("Test route with param: {}", path_param)
}

async fn route_serialize_testparam(Path(testparam): Path<String>) -> String {
    let data = DataMessage {id: String::from("1"), content: testparam};

    format!("{:?}", data)
}
