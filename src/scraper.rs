async fn browse_website(url: &str) -> Result<(), CmdError> {
    // Start a new WebDriver session
    let mut client = Client::new("http://:4444").await?;

    // Navigate to the specified URL
    client.goto(url).await?;

    // Get the page title
    let title = client.title().await?;
    println!("Title: {}", title);

    // Find and print all links on the page
    match client.find_all(Locator::Css("a")).await {
        Ok(elements) => {
            for element in elements {
                if let Ok(link) = element.get_attribute("href").await {
                    println!("Link: {}", link);
                }
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
