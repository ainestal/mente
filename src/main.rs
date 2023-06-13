mod db_logging;
mod openai;

use anyhow::{Context, Result};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

use db_logging::log_message;
use openai::chat_gpt::ChatGPT;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let model = std::env::var("OPENAI_MODEL")?;
    let key = std::env::var("OPENAI_API_KEY")?;
    let db_url = std::env::var("DATABASE_URL")?;
    let session_id = Uuid::new_v4().to_string();
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let mut gpt = ChatGPT::new(key, session_id.clone(), model)?;

    println!("Initialised GPT-4 chatbot. Enter your message to start a conversation.");
    println!("Using:");
    println!("- Model: {}", gpt.model);
    println!("- Session ID: {}", gpt.session_id);
    println!("You can quit by pressing Ctrl+C (linux), or Cmd+C (Mac).");
    println!("--------------------------------------");
    loop {
        println!("- Enter your message and press Enter:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .context("Failed to read your input")?;

        log_message(
            "user".to_string(),
            input.clone(),
            gpt.session_id.clone(),
            &pool,
        )
        .await?;

        println!("- AI:");
        let answer = gpt
            .completion(input)
            .await
            .context("Could not get an answer from GPT")?;

        log_message(
            answer.role,
            answer.content.clone(),
            gpt.session_id.clone(),
            &pool,
        )
        .await?;

        println!("{}", format!("{}", answer.content));
        println!("--------------------------------------");
    }
}
