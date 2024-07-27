use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use axum::{http::header, response::{Html, IntoResponse}, routing::get, Router};
use rand::Rng;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

#[derive(Debug, Serialize)]
struct TestRecord {
    title: String,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: surrealdb::sql::Thing,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Connect to the server
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("test").use_db("test").await?;

    // spawn a extra thread which generates periodically some timestamp updates.
    tokio::spawn(async move {
        // build our application with a route
        let app = Router::new()
            .route("/", get(handler))
            .route("/surrealdb-beta9.js", get(js_handler));

        // run it
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await.unwrap();

        println!("listening on {}", listener.local_addr().unwrap());

        axum::serve(listener, app).await.unwrap();
    });

    const NUM_OF_RECS: u64 = 5;

    loop {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        println!("timestamp: {}", now);

        for i in 0..NUM_OF_RECS {
            let _: Option<Record> = db.update(("tests", i+1))
                .merge(TestRecord{
                    title: format!("Test Record {}", i+1),
                    timestamp: now,
                }).await.unwrap();
        }

        let mut rng = rand::thread_rng();
        let num = rng.gen_range(1..10);
        println!("Sleeping {}", num);
        tokio::time::sleep(tokio::time::Duration::from_secs(num)).await;
    }

    // Ok(())
}

async fn handler() -> Html<&'static str> {
    let content = include_str!("index.html");

    Html(content)
}

async fn js_handler() -> impl IntoResponse {
    let content = include_str!("surrealdb-beta9.js");

    ([(header::CONTENT_TYPE, "application/javascript; charset=utf-8")], content)
}
