//! StarVote API - Basic Stats API
extern crate dotenv;
use axum::{routing::get, Router, response::Json, extract::State};
use serde::Serialize;
use sqlx::any::install_default_drivers;
use sqlx::{Any, Pool, Row};
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

async fn stats(pool: &Pool<Any>) -> Stats {
    let guilds: (i64,) = sqlx::query_as("SELECT COUNT(DISTINCT guild_id) FROM polls")
        .fetch_one(pool).await.unwrap_or((0,));
   
    let polls: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM polls")
        .fetch_one(pool).await.unwrap_or((0,));
   
    let votes: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM votes")
        .fetch_one(pool).await.unwrap_or((0,));
   
    let polls_by_type = sqlx::query("SELECT voting_method, COUNT(*) as count FROM polls GROUP BY voting_method")
        .fetch_all(pool).await.unwrap_or_default()
        .into_iter().map(|row| (row.get::<String, _>("voting_method"), row.get::<i64, _>("count") as u64)).collect();
   
    // let polls_per_guild = sqlx::query("SELECT guild_id, COUNT(*) as count FROM polls GROUP BY guild_id")
    //     .fetch_all(pool).await.unwrap_or_default()
    //     .into_iter().map(|row| (row.get::<String,_>("guild_id"), row.get::<i64,_>("count") as u64)).collect();
   
    Stats {
        guilds: guilds.0 as u64,
        polls: polls.0 as u64,
        votes: votes.0 as u64,
        polls_by_type,
        // polls_per_guild,
    }
}

async fn stats_handler(State(pool): State<Arc<Pool<Any>>>) -> Json<Stats> {
    Json(stats(&pool).await)
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    install_default_drivers();
    
    // Create an Any pool directly
    let pool: Pool<Any> = Pool::connect(&db_url)
        .await
        .expect("Failed to connect to database");
    
    println!("Connected to database");
   
    let shared_pool = Arc::new(pool);
   
    let app = Router::new()
        .route("/stats", get(stats_handler))
        .with_state(shared_pool);
   
    let host = env::var("HOST")
        .ok()
        .and_then(|s| s.parse::<std::net::IpAddr>().ok())
        .unwrap_or_else(|| "127.0.0.1".parse().unwrap());

    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from((host, port));
    println!("StarVote API running at http://{}/stats", addr);
   
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
   
    axum::serve(listener, app)
        .await
        .unwrap();
}