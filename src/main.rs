use axum::{extract::Path, routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = app();
    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    Router::new().route("/:name", get(handler))
}

async fn handler(Path(name): Path<String>) -> String {
    format!("Hey👋, {}\n", name)
}

#[cfg(test)]
mod tests {
    use crate::app;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::util::ServiceExt; // for `collect`

    #[tokio::test]
    pub async fn test_hello() {
        let app = app();
        let name = "world";
        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/{}", name))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], format!("Hey👋, {}\n", name).as_bytes());
    }
}
