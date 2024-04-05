use axum::{
    routing::get,
    //routing::post,
    Router,
};
mod routes;
use routes::product::print_hello;
use dotenv::dotenv;

#[tokio::main]
async fn main(){

    dotenv().ok();

    let app = Router::new()
        .route("/products", get(print_hello()));
    let port = std::env::var("PORT").expect("PORT must be set");


    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}",port))
        .await
        .unwrap();
    println!("the server is running on port:{}",port);
    axum::serve(listener, app)
        .await
        .unwrap();
}
