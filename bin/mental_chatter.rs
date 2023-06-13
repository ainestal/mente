use anyhow::{Context, Result};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

use mente::db_logging::log_message;
use mente::openai::chat_gpt::{ChatGPT, Message};
use mente::personality::{Personalities, Personality};

// Initialize the personalities
fn init_personalities(model: String) -> Personalities {
    let mut personalities = Personalities::new();

    let alice = Personality::new(
        "Alice".to_string(),
        "You are an energetic young person who is feeling down. You are looking for a way to feel better.".to_string(),
        model.clone());

    let bob = Personality::new(
    "Bob".to_string(),
    "You are a wise old monk that heavily struggled in life and now can see and really understand what is happiness.".to_string(),
    model.clone());

    personalities.add_personality(alice);
    personalities.add_personality(bob);
    personalities
}

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

    // ---------------------------------------
    // In this example, we only use two personalities and they will take turns to respond
    let mente = init_personalities(model.clone());

    println!("Initialised mental chatter, a fun experiment with GPT-4.");
    println!("Using:");
    println!("- Model: {}", model);
    println!("- Session ID: {}", gpt.session_id);
    println!("- Personalities: {}", mente.personalities.len());
    for personality in mente.personalities.iter() {
        println!("  - {}", personality.name);
    }
    println!("- You can switch between personalities by typing 'switch to <name>'");
    println!("You can quit by pressing Ctrl+C (linux), or Cmd+C (Mac).");
    println!("--------------------------------------");

    let mut index = 0;

    let initial_prompt = "What seems to be the most common problem that humans have?".to_string();
    let mut anwser = Message {
        role: "user".to_string(),
        content: initial_prompt,
    };
    println!("{}", anwser.content);

    loop {
        let mut personality = mente.get_personality_by_index(index).unwrap();
        println!("- {}:", personality.name);

        // Change the last answer to have user as the role
        anwser.role = "user".to_string();
        // Push the last answer to the context of the personality that will reply next
        personality.push_to_context(anwser.clone());

        anwser = gpt
            .completion(&personality.context)
            .await
            .context("Could not get an answer from GPT")?;

        log_message(
            "assistant".to_string(),
            anwser.content.clone(),
            gpt.session_id.clone(),
            personality.name.clone(),
            &pool,
        )
        .await?;

        // Add the last answer to the context of the personality that replied
        personality.push_to_context(anwser.clone());

        println!("{}", format!("{}", anwser.content));
        println!("--------------------------------------");

        index = (index + 1) % mente.personalities.len();
    }
}
