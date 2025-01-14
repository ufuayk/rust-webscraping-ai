use reqwest;
use std::fs;
use std::io::Write;
use url::Url;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://example.com/";
    let base_url = Url::parse(url).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    let output_dir = "downloaded_content";
    fs::create_dir_all(output_dir).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    println!("Created directory: '{}'", output_dir);
    
    let client = reqwest::Client::new();
    println!("Downloading content from '{}'...", url);
    
    let response = client.get(url).send().await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    let html_content = response.text().await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    let html_path = format!("{}/index.html", output_dir);
    fs::write(&html_path, &html_content)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    println!("Saved HTML content: {}", html_path);
    
    let document = Html::parse_document(&html_content);
    
    // Download CSS files
    if let Ok(css_selector) = Selector::parse("link[rel='stylesheet']") {
        for element in document.select(&css_selector) {
            if let Some(css_url) = element.value().attr("href") {
                if let Ok(absolute_url) = base_url.join(css_url) {
                    let file_name = absolute_url.path_segments()
                        .and_then(|segments| segments.last())
                        .unwrap_or("style.css");
                    
                    println!("Downloading CSS: {}", absolute_url);
                    if let Ok(resp) = client.get(absolute_url.as_str()).send().await {
                        if let Ok(css_content) = resp.bytes().await {
                            let css_path = format!("{}/css_{}", output_dir, file_name);
                            if let Ok(mut file) = fs::File::create(&css_path) {
                                if let Err(e) = file.write_all(&css_content) {
                                    eprintln!("Error writing CSS: {}", e);
                                } else {
                                    println!("Saved CSS: {}", css_path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Download JavaScript files
    if let Ok(js_selector) = Selector::parse("script[src]") {
        for element in document.select(&js_selector) {
            if let Some(js_url) = element.value().attr("src") {
                if let Ok(absolute_url) = base_url.join(js_url) {
                    let file_name = absolute_url.path_segments()
                        .and_then(|segments| segments.last())
                        .unwrap_or("script.js");
                    
                    println!("Downloading JavaScript: {}", absolute_url);
                    if let Ok(resp) = client.get(absolute_url.as_str()).send().await {
                        if let Ok(js_content) = resp.bytes().await {
                            let js_path = format!("{}/js_{}", output_dir, file_name);
                            if let Ok(mut file) = fs::File::create(&js_path) {
                                if let Err(e) = file.write_all(&js_content) {
                                    eprintln!("Error writing JavaScript: {}", e);
                                } else {
                                    println!("Saved JavaScript: {}", js_path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Download Images
    if let Ok(img_selector) = Selector::parse("img") {
        for element in document.select(&img_selector) {
            if let Some(img_url) = element.value().attr("src") {
                if let Ok(absolute_url) = base_url.join(img_url) {
                    let file_name = absolute_url.path_segments()
                        .and_then(|segments| segments.last())
                        .unwrap_or("image.jpg");
                    
                    println!("Downloading Image: {}", absolute_url);
                    if let Ok(resp) = client.get(absolute_url.as_str()).send().await {
                        if let Ok(img_content) = resp.bytes().await {
                            let img_path = format!("{}/img_{}", output_dir, file_name);
                            if let Ok(mut file) = fs::File::create(&img_path) {
                                if let Err(e) = file.write_all(&img_content) {
                                    eprintln!("Error writing image: {}", e);
                                } else {
                                    println!("Saved image: {}", img_path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("Process completed! All files are in '{}' directory.", output_dir);
    Ok(())
}