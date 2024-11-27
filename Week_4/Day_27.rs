async fn scrape_url(&self, url: String, retry_count: u32) -> ScrapedData {
    let start_time = Instant::now();

    // Introduce delay between requests to be polite to servers
    if retry_count > 0 {
        time::sleep(self.config.delay_between_requests).await;
    }

    let result = async {
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
            url,
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
