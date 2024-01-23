use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // The URL you want to request
    let url = "https://phys.org/rss-feed/physics-news/rss/";

    // Build a client with a custom user-agent header
    let client = reqwest::Client::builder()
        .user_agent("Rust Science Feed ")
        .build()?;

    // Perform a GET request to the URL using the custom client
    let response = client.get(url).send().await?;

    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        // Read the response body as a string
        let html = response.text().await?;

        // Print the HTML content
        println!("{}", html);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}