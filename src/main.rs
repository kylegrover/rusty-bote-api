//! StarVote API - Basic Stats API
use axum::{routing::get, Router, response::Json, extract::State};
use serde::Serialize;
use sqlx::{Pool, Row, Sqlite};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Serialize)]
struct Stats {
    guilds: u64,
    polls: u64,
    votes: u64,
    polls_by_type: Vec<(String, u64)>,
    // polls_per_guild: Vec<(String, u64)>,
}

async fn stats(pool: Pool<Sqlite>) -> Stats {
    let guilds: (i64,) = sqlx::query_as("SELECT COUNT(DISTINCT guild_id) FROM polls")
        .fetch_one(&pool).await.unwrap_or((0,));
   
    let polls: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM polls")
        .fetch_one(&pool).await.unwrap_or((0,));
   
    let votes: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM votes")
        .fetch_one(&pool).await.unwrap_or((0,));
   
    let polls_by_type = sqlx::query("SELECT voting_method, COUNT(*) as count FROM polls GROUP BY voting_method")
        .fetch_all(&pool).await.unwrap_or_default()
        .into_iter().map(|row| (row.get::<String,_>("voting_method"), row.get::<i64,_>("count") as u64)).collect();
   
    // let polls_per_guild = sqlx::query("SELECT guild_id, COUNT(*) as count FROM polls GROUP BY guild_id")
    //     .fetch_all(&pool).await.unwrap_or_default()
    //     .into_iter().map(|row| (row.get::<String,_>("guild_id"), row.get::<i64,_>("count") as u64)).collect();
   
    Stats {
        guilds: guilds.0 as u64,
        polls: polls.0 as u64,
        votes: votes.0 as u64,
        polls_by_type,
        // polls_per_guild,
    }
}

async fn stats_handler(State(pool): State<Arc<Pool<Sqlite>>>) -> Json<Stats> {
    Json(stats((*pool).clone()).await)
}

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:rusty_bote_dev.db".to_string());
   
    let pool = sqlx::SqlitePool::connect(&db_url)
        .await
        .expect("Failed to connect to DB");
    
    let shared_pool = Arc::new(pool);
   
    let app = Router::new()
        .route("/stats", get(stats_handler))
        .with_state(shared_pool);
   
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("StarVote API running at http://{}/stats", addr);
   
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    
    axum::serve(listener, app)
        .await
        .unwrap();
}