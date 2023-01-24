use axum_project::app;

#[tokio::main]
async fn main() {
    // build our application with a single route
    
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}
