use std::error::Error;
use std::time::Instant;
use reqwest::Client;
use scraper::{Html, Selector};
use tokio::sync::mpsc;
use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::fs;

// Structure to hold scraped data
#[derive(Debug, Serialize, Deserialize)]
struct ScrapedData {
    url: String,
    title: String,
    description: Option<String>,
    links: Vec<String>,
    status_code: u16,
    fetch_time_ms: u128,
}

// Configuration for the scraper
#[derive(Debug)]
struct ScraperConfig {
    concurrent_requests: usize,
    timeout_seconds: u64,
    max_retries: u32,
}

impl Default for ScraperConfig {
    fn default() -> Self {
        Self {
            concurrent_requests: 5,
            timeout_seconds: 30,
            max_retries: 3,
        }
    }
}

// Main scraper struct
struct WebScraper {
    client: Client,
    config: ScraperConfig,
}

impl WebScraper {
    fn new(config: ScraperConfig) -> Result<Self, Box<dyn Error>> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()?;

        Ok(Self { client, config })
    }

async fn scrape_url(&self, url: String, retry_count: u32) -> Result<ScrapedData, Box<dyn Error>> {
        let start_time = Instant::now();
        
        let response = match self.client.get(&url).send().await {
            Ok(resp) => resp,
            Err(e) => {
                if retry_count < self.config.max_retries {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    // Box the future for recursion
                    return Box::pin(self.scrape_url(url, retry_count + 1)).await;
                } else {
                    return Err(Box::new(e));
                }
            }
        };

        let status = response.status().as_u16();
        let html = response.text().await?;
        let document = Html::parse_document(&html);

        let title_selector = Selector::parse("title").unwrap();
        let title = document
            .select(&title_selector)
            .next()
            .map(|element| element.text().collect::<String>())
            .unwrap_or_default();

        let desc_selector = Selector::parse("meta[name='description']").unwrap();
        let description = document
            .select(&desc_selector)
            .next()
            .and_then(|element| element.value().attr("content"))
            .map(String::from);

        let link_selector = Selector::parse("a[href]").unwrap();
        let links: Vec<String> = document
            .select(&link_selector)
            .filter_map(|element| element.value().attr("href"))
            .map(String::from)
            .collect();

        Ok(ScrapedData {
            url,
            title,
            description,
            links,
            status_code: status,
            fetch_time_ms: start_time.elapsed().as_millis(),
        })
    }

    async fn scrape_urls(&self, urls: Vec<String>) -> Result<Vec<ScrapedData>, Box<dyn Error>> {
        let (tx, mut rx) = mpsc::channel(self.config.concurrent_requests);

        // Process URLs concurrently with rate limiting
        let process_handle = tokio::spawn(async move {
            let mut results = Vec::new();
            while let Some(data) = rx.recv().await {
                results.push(data);
            }
            results
        });

        // Create stream of URLs and process them
        stream::iter(urls)
            .map(|url| {
                let tx = tx.clone();
                
                async move {
                    match self.scrape_url(url, 0).await {
                        Ok(data) => {
                            if let Err(e) = tx.send(data).await {
                                eprintln!("Error sending data: {}", e);
                            }
                        }
                        Err(e) => eprintln!("Error scraping URL: {}", e),
                    }
                }
            })
            .buffer_unordered(self.config.concurrent_requests)
            .collect::<Vec<()>>()
            .await;

        // Drop sender to close channel
        drop(tx);

        // Wait for all results to be processed
        let results = process_handle.await?;
        Ok(results)
    }

    async fn save_results(&self, results: Vec<ScrapedData>, filename: &str) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(&results)?;
        fs::write(filename, json).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Example URLs to scrape
    let urls = vec![
        "https://www.rust-lang.org".to_string(),
        "https://docs.rs".to_string(),
        "https://crates.io".to_string(),
    ];

    // Create scraper with custom configuration
    let config = ScraperConfig {
        concurrent_requests: 3,
        timeout_seconds: 20,
        max_retries: 2,
    };

    let scraper = WebScraper::new(config)?;
    
    println!("Starting scrape of {} URLs...", urls.len());
    let start_time = Instant::now();

    // Perform scraping
    let results = scraper.scrape_urls(urls).await?;

    // Print summary
    println!("Scraping completed in {}ms", start_time.elapsed().as_millis());
    println!("Successfully scraped {} sites", results.len());

    // Save results to file
    scraper.save_results(results, "scraping_results.json").await?;
    println!("Results saved to scraping_results.json");

    Ok(())
}
