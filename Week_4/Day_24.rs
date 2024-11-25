use std::error::Error;
use std::time::Instant;
use reqwest::Client;
use scraper::{Html, Selector};
use tokio::sync::mpsc;
use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ScrapedData {
    url: String,
    title: String,
    description: Option<String>,
    first_paragraph: Option<String>,
    status_code: u16,
    fetch_time_ms: u128,
}

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
                    return Box::pin(self.scrape_url(url.clone(), retry_count + 1)).await;
                } else {
                    return Err(Box::new(e));
                }
            }
        };

        let status_code = response.status().as_u16();
        let fetch_time_ms = start_time.elapsed().as_millis();
        let body = response.text().await?;

        let document = Html::parse_document(&body);

        let title_selector = Selector::parse("title").unwrap();
        let description_selector = Selector::parse("meta[name=\"description\"]").unwrap();
        let paragraph_selector = Selector::parse("p").unwrap();

        let title = document
            .select(&title_selector)
            .next()
            .map(|e| e.inner_html())
            .unwrap_or_default();

        let description = document
            .select(&description_selector)
            .next()
            .and_then(|e| e.value().attr("content"))
            .map(|s| s.to_string());

        let first_paragraph = document
            .select(&paragraph_selector)
            .next()
            .map(|e| e.inner_html());

        Ok(ScrapedData {
            url,
            title,
            description,
            first_paragraph,
            status_code,
            fetch_time_ms,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = ScraperConfig::default();
    let scraper = WebScraper::new(config)?;

    let urls = vec![
        "https://example.com".to_string(),
        "https://www.rust-lang.org".to_string(),
    ];

    let (tx, mut rx) = mpsc::channel(100);

    stream::iter(urls)
        .for_each_concurrent(scraper.config.concurrent_requests, |url| {
            let tx = tx.clone();
            let scraper = &scraper;
            async move {
                if let Ok(data) = scraper.scrape_url(url, 0).await {
                    tx.send(data).await.unwrap();
                }
            }
        })
        .await;

    drop(tx);

    while let Some(data) = rx.recv().await {
        println!("{:#?}", data);
    }

    Ok(())
}
