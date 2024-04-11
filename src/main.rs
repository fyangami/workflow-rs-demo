use axum::{extract::Path, routing::get, Extension, Router};
use fred::{
    clients::RedisClient,
    interfaces::KeysInterface,
    types::{Builder, RedisConfig},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // build our application with a route
    let app = app(init_rdb("redis://redis:6379/1")?);
    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

fn app(rdb: RedisClient) -> Router {
    Router::new()
        .route("/greeting/:name", get(greeting_handler))
        .route("/set/:val", get(rdb_set))
        .route("/get", get(rdb_get))
        .layer(Extension(rdb))
}

/// say hello
async fn greeting_handler(Path(name): Path<String>) -> String {
    format!("Hey👋, {}\n", name)
}

static REDIS_KEY: &str = "SET_KEY";

/// put a val to redis
async fn rdb_set(Extension(rdb): Extension<RedisClient>, Path(val): Path<String>) {
    let _ = rdb
        .set::<i32, _, _>(REDIS_KEY, val.as_str(), None, None, false)
        .await;
}

/// get a val from redis
async fn rdb_get(Extension(rdb): Extension<RedisClient>) -> String {
    if let Ok(get) = rdb.get::<Option<String>, _>(REDIS_KEY).await {
        if let Some(val) = get {
            return val;
        }
    }
    "nil".to_owned()
}

/// initializing redis client
fn init_rdb(url: &str) -> Result<RedisClient, anyhow::Error> {
    let rdb = Builder::from_config(RedisConfig::from_url(url)?).build()?;
    Ok(rdb)
}

#[cfg(test)]
mod tests {
    use crate::{app, init_rdb};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::util::ServiceExt; // for `collect`

    #[tokio::test]
    pub async fn test_greeting() {
        let app = app(init_rdb("redis://redis:6379/1").unwrap());
        let name = "world";
        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/greeting/{}", name))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], format!("Hey👋, {}\n", name).as_bytes());
    }

    #[tokio::test]
    pub async fn test_set_val() {
        let app = app(init_rdb("redis://redis:6379/1").unwrap());
        let val = "set a val";
        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/set/{}", val))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    pub async fn test_get_val() {
        let app = app(init_rdb("redis://redis:6379/1").unwrap());
        let val = "set a val";
        let response = app
            .oneshot(Request::builder().uri("/get").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], val.as_bytes());
    }
}
