extern crate core;

mod routing;
mod data;

const PORT: u16 = 4200;

#[tokio::main]
async fn main() {
    println!("Welcome to rust-intro");

    loop {
        let routes;
        {
            routes = routing::init_routes().await;
        }
        println!("Starting server on port: {}", PORT);
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT)).await.unwrap();
        let _ = axum::serve(listener, routes).await;
    }
}