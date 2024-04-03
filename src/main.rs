use axum::{
    routing::get,
    Router,
};


#[tokio::main]
async fn main(){

    let app = Router::new()
        .route("/", get(|| async {"Hello World"}))
        .route("/user", get(|| async {"These are all users"}));
    let port = 3000;


    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}",port))
        .await
        .unwrap();
    println!("the server is running on port:{}",port);
    axum::serve(listener, app)
        .await
        .unwrap();
}
