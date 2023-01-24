use axum_project::app;
use axum_test_helper::TestClient;
use http::StatusCode;
use tokio::test;

#[test]
async fn test() {
    let client = TestClient::new(app());

    let res = client.get("/").send().await;

    assert_eq!(res.status(), StatusCode::OK);
}