mod routes;
mod recording;

#[tokio::main]
async fn main() {
    let app = routes::create_router();

    println!("Starting backend on http://0.0.0.0:3000");
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
