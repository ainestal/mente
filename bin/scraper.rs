use std::env;

use anyhow::Result;
use fantoccini::{ClientBuilder, Locator};
use tokio;

async fn browse_website(url: &str) -> Result<()> {
    // Start a new WebDriver session
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");

    // Navigate to the specified URL
    client.goto(url).await?;

    // Get the page title
    let title = client.title().await?;
    println!("Title: {}", title);

    // Locator XPath that finds all the curl code examples in the page
    // let xpath = "//*[contains(@class, 'language-bash')]";

    // Find all the examples using curl in the page
    // match client.find_all(Locator::XPath(xpath)).await {
    //     Ok(elements) => {
    //         for element in elements {
    //             let text = element.text().await?;

    //             if text.contains("https://api.stripe.com") {
    //                 println!("{:?}", text);
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         println!("Error finding code: {:?}", e);
    //     }
    // }

    // Find and print all links on the page
    match client.find_all(Locator::Css("a")).await {
        Ok(elements) => {
            for element in elements {
                if let Ok(link) = element.attr("href").await {
                    if link.is_some() {
                        println!("Link: {}", link.unwrap());
                    } else {
                        continue;
                    }
                } else {
                    continue;
                };
            }
        }
        Err(e) => {
            println!("Error finding links: {:?}", e);
        }
    }

    // Close the browser session
    let _ = client.close().await;

    Ok(())
}

#[tokio::main]
async fn main() {
    // Get arguments
    let args: Vec<String> = env::args().collect();
    let url = if args[1].is_empty() {
        "https://example.com".to_string()
    } else {
        args[1].clone()
    };

    if let Err(e) = browse_website(&url).await {
        println!("Error: {:?}", e);
    }
}
