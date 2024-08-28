use std::sync::{Arc, RwLock};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    response::Response,
    Router
};
use rust_intro::rust_intro::DataMessage;

use crate::data::DataHandler;

pub(crate) async fn init_routes() -> Router {
    let handler = Arc::new(RwLock::new(DataHandler::new()));
    Router::new()
        .route("/", get(route_root))
        .route("/test", get(route_test))
        .route("/test/:path_param", get(route_test_with_param))
        .route("/proto/serialize/:path_param", get(route_serialize_testparam))
        .route("/proto/:data_message_id/:message", post(route_add_data_msg))
        .route("/proto/:data_message_id", get(route_get_data_msg))
        .route("/proto", get(route_get_all_data_msg))
        .with_state(handler)
}

async fn route_root(_: State<Arc<RwLock<DataHandler>>>) -> &'static str {
    "Welcome to rust-intro"
}

async fn route_test(_: State<Arc<RwLock<DataHandler>>>) -> &'static str {
    "Test route"
}

async fn route_test_with_param(_: State<Arc<RwLock<DataHandler>>>, Path(path_param): Path<String>) -> String {
    format!("Test route with param: {}", path_param)
}

async fn route_serialize_testparam(_: State<Arc<RwLock<DataHandler>>>, Path(testparam): Path<String>) -> String {
    let data = DataMessage { id: String::from("1"), content: testparam };

    format!("{:?}", data)
}

async fn route_add_data_msg(State(state): State<Arc<RwLock<DataHandler>>>, Path((id, content)): Path<(String, String)>) -> StatusCode {
    let mut lock = state.write().expect("could not get write lock");

    lock.add(DataMessage { id, content });

    StatusCode::OK
}

async fn route_get_data_msg(State(state): State<Arc<RwLock<DataHandler>>>, Path(id): Path<String>) -> Response<String> {
    let lock = state.read().expect("could not get read lock");

    if let Some(msg) = lock.get(&id) {
        Response::builder().status(StatusCode::OK).body(format!("{:?}", msg)).expect("could not create http response")
    } else {
        Response::builder().status(StatusCode::NOT_FOUND).body(format!("no message found with id {:?}", id)).expect("could not create http response")
    }
}

async fn route_get_all_data_msg(State(state): State<Arc<RwLock<DataHandler>>>) -> Response<String> {
    let lock = state.read().expect("could not get read lock");

    let mut response = String::new();

    for msg in lock.get_all() {
        response += &format!("{:?}\n", msg)
    }

    Response::builder().status(StatusCode::OK).body(response).expect("could not create http response")
}
