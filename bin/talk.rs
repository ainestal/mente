use anyhow::{Context, Result};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

use mente::db_logging::log_message;
use mente::openai::chat_context::ChatContext;
use mente::openai::chat_gpt::{ChatGPT, Message};

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

    let mut gpt = ChatGPT::new(key, session_id.clone())?;
    let mut chat_context = ChatContext::new(model.clone());

    println!("Initialised GPT-4 chatbot. Enter your message to start a conversation.");
    println!("Using:");
    println!("- Model: {}", chat_context.model);
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
            "user".to_string(),
            &pool,
        )
        .await?;

        chat_context.push(Message {
            role: "user".to_string(),
            content: input.clone(),
        });

        println!("- AI:");
        let answer = gpt
            .completion(&chat_context)
            .await
            .context("Could not get an answer from GPT")?;

        log_message(
            answer.role,
            answer.content.clone(),
            gpt.session_id.clone(),
            "assistant".to_string(),
            &pool,
        )
        .await?;

        println!("{}", format!("{}", answer.content));
        println!("--------------------------------------");
    }
}
