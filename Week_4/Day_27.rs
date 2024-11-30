use std::error::Error;
use std::time::{Duration, Instant};
use reqwest::Client;
use scraper::{Html, Selector};
use tokio::sync::mpsc;
use tokio::time;
use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
struct ScrapedData {
    url: String,
    title: String,
    description: Option<String>,
    first_paragraph: Option<String>,
    links: Vec<String>,
    status_code: u16,
    fetch_time_ms: u128,
    error: Option<String>,
}

#[derive(Debug, Clone)]
struct ScraperConfig {
    concurrent_requests: usize,
    timeout_seconds: u64,
    max_retries: u32,
    delay_between_requests: Duration,
}

impl Default for ScraperConfig {
    fn default() -> Self {
        Self {
            concurrent_requests: 10,
            timeout_seconds: 30,
            max_retries: 3,
            delay_between_requests: Duration::from_millis(500),
        }
    }
}

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

    async fn scrape_url(&self, url: String, retry_count: u32) -> ScrapedData {
        let start_time = Instant::now();
    
        // Introduce delay between requests to be polite to servers
        if retry_count > 0 {
            time::sleep(self.config.delay_between_requests).await;
        }
    
        let result: Result<ScrapedData, Box<dyn Error>> = async {
            let response = self.client.get(&url).send().await?;
            let status_code = response.status().as_u16();
            let body = response.text().await?;
    
            let document = Html::parse_document(&body);
    
            // Selectors
            let title_selector = Selector::parse("title").unwrap();
            let description_selector = Selector::parse("meta[name=\"description\"]").unwrap();
            let paragraph_selector = Selector::parse("p").unwrap();
            let link_selector = Selector::parse("a").unwrap();
    
            // Extract title
            let title = document
                .select(&title_selector)
                .next()
                .map(|e| e.inner_html())
                .unwrap_or_default();
    
            // Extract meta description
            let description = document
                .select(&description_selector)
                .next()
                .and_then(|e| e.value().attr("content"))
                .map(|s| s.to_string());
    
            // Extract first paragraph
            let first_paragraph = document
                .select(&paragraph_selector)
                .next()
                .map(|e| e.inner_html());
    
            // Extract links
            let links = document
                .select(&link_selector)
                .filter_map(|e| e.value().attr("href"))
                .filter(|link| link.starts_with("http"))
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
    
            Ok(ScrapedData {
                url: url.clone(),
                title,
                description,
                first_paragraph,
                links,
                status_code,
                fetch_time_ms: start_time.elapsed().as_millis(),
                error: None,
            })
        }
        .await;
    
        match result {
            Ok(data) => data,
            Err(e) => {
                if retry_count < self.config.max_retries {
                    // Use Box::pin for recursive async call
                    return Box::pin(self.scrape_url(url.clone(), retry_count + 1)).await;
                }
    
                ScrapedData {
                    url,
                    title: String::new(),
                    description: None,
                    first_paragraph: None,
                    links: vec![],
                    status_code: 0,
                    fetch_time_ms: start_time.elapsed().as_millis(),
                    error: Some(format!("Failed to scrape: {}", e)),
                }
            }
        }
    }

    async fn run_scraping_tasks(&self, urls: Vec<String>) -> Vec<ScrapedData> {
        let (tx, mut rx) = mpsc::channel(self.config.concurrent_requests);

        // Concurrent scraping with stream
        stream::iter(urls)
            .for_each_concurrent(self.config.concurrent_requests, |url| {
                let tx = tx.clone();
                let scraper = self.clone();
                async move {
                    let data = scraper.scrape_url(url, 0).await;
                    tx.send(data).await.unwrap();
                }
            })
            .await;

        // Close sender
        drop(tx);

        // Collect results
        let mut results = Vec::new();
        while let Some(data) = rx.recv().await {
            results.push(data);
        }

        results
    }
}

// Implement Clone for WebScraper to use in concurrent contexts
impl Clone for WebScraper {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            config: self.config.clone(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Configuration
    let config = ScraperConfig {
        concurrent_requests: 10,
        timeout_seconds: 30,
        max_retries: 3,
        delay_between_requests: Duration::from_millis(500),
    };

    // Initialize scraper
    let scraper = WebScraper::new(config)?;

    // URLs to scrape
    let urls = vec![
        "https://www.rust-lang.org".to_string(),
        "https://docs.rs".to_string(),
        "https://crates.io".to_string(),
        "https://www.mozilla.org".to_string(),
        "https://blog.rust-lang.org".to_string(),
    ];

    // Run scraping tasks
    let results = scraper.run_scraping_tasks(urls).await;

    // Analyze and print results
    println!("Scraping Complete. Total Pages: {}", results.len());
    
    // Categorize results
    let mut successful_scrapes = 0;
    let mut failed_scrapes = 0;
    let mut total_links = 0;

    for (index, result) in results.iter().enumerate() {
        println!("\nPage {}: {}", index + 1, result.url);
        
        if result.error.is_none() {
            successful_scrapes += 1;
            total_links += result.links.len();
            
            println!("  Title: {}", result.title);
            println!("  Description: {}", result.description.clone().unwrap_or_default());
            println!("  Links Found: {}", result.links.len());
        } else {
            failed_scrapes += 1;
            println!("  Error: {}", result.error.clone().unwrap());
        }
    }

    // Summary
    println!("\n--- Scraping Summary ---");
    println!("Successful Scrapes: {}", successful_scrapes);
    println!("Failed Scrapes: {}", failed_scrapes);
    println!("Total Links Discovered: {}", total_links);

    Ok(())
}
