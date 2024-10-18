use crate::args::Args;
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Url;
use scraper::{Html, Selector};
use std::time::Duration;
use std::{collections::HashSet, path::Path};
use tokio::fs;
use tokio::sync::Mutex;

mod args;

lazy_static! {
    static ref DOWNLOADED_FILES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

fn get_normalized_path(url: &Url) -> String {
    let mut path = url.path().trim_start_matches('/').to_string();
    if path.is_empty() || path.ends_with('/') {
        path.push_str("index.html");
    }
    path
}

async fn download_page(url: &Url) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url.clone()).await?;
    Ok(response.text().await?)
}

fn extract_links(base_url: &Url, html: &str) -> (HashSet<Url>, Vec<(String, Url)>) {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a[href], link[href], img[src], script[src]").unwrap();
    let mut internal_links = HashSet::new();
    let mut external_assets = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element
            .value()
            .attr("href")
            .or_else(|| element.value().attr("src"))
        {
            if let Ok(link_url) = base_url.join(href) {
                if link_url.domain() == base_url.domain() {
                    internal_links.insert(link_url);
                } else {
                    external_assets.push((href.to_string(), link_url));
                }
            }
        }
    }

    (internal_links, external_assets)
}

async fn download_static_content(url: &Url) -> Result<String, reqwest::Error> {
    let file_name = get_normalized_path(url);
    let local_path = format!("downloads/static/{}", file_name);

    let mut downloaded_files = DOWNLOADED_FILES.lock().await;
    if downloaded_files.contains(&local_path) {
        return Ok(local_path.clone());
    }

    let response = reqwest::get(url.clone()).await?;
    let content = response.bytes().await?;

    let dir = Path::new(&local_path).parent().unwrap();
    fs::create_dir_all(dir).await.unwrap();
    fs::write(&local_path, &content).await.unwrap();

    downloaded_files.insert(local_path.clone());

    Ok(local_path)
}

fn replace_external_links(html: &str, replacements: &[(&String, String)]) -> String {
    let mut updated_html = html.to_string();
    for (old_link, new_link) in replacements {
        updated_html = updated_html.replace(*old_link, new_link);
    }
    updated_html
}

async fn modify_and_download_css(
    url: &Url,
    css_content: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url_regex =
        Regex::new(r#"url\((?P<quote>['"]?)(?P<link>https?://[^)'"]+)(?P<quote_end>['"]?)\)"#)
            .unwrap();
    let mut modified_css = css_content.to_string();
    let mut replacements = Vec::new();

    for cap in url_regex.captures_iter(css_content) {
        if let Some(link) = cap.name("link") {
            if let Ok(resource_url) = Url::parse(link.as_str()) {
                match download_static_content(&resource_url).await {
                    Ok(local_path) => {
                        let new_url = format!("url('{}')", local_path);
                        replacements.push((link.as_str().to_string(), new_url));
                    }
                    Err(e) => eprintln!("Failed to download resource {:?}: {}", link, e),
                }
            }
        }
    }

    for (old_url, new_url) in replacements {
        modified_css = modified_css.replace(&old_url, &new_url);
    }

    Ok(modified_css)
}

async fn crawl_website(start_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let start_url = Url::parse(start_url)?;
    let mut visited_pages = HashSet::new();
    let mut to_visit = vec![start_url.clone()];

    while let Some(url) = to_visit.pop() {
        if visited_pages.contains(&url) {
            continue;
        }

        match download_page(&url).await {
            Ok(html) => {
                let normalized_path = get_normalized_path(&url);
                let path = format!("downloads/pages/{}", normalized_path);
                let dir = Path::new(&path).parent().unwrap();
                fs::create_dir_all(dir).await?;

                let (internal_links, external_assets) = extract_links(&url, &html);

                let mut replacements = Vec::new();
                for (original_link, external_url) in external_assets.iter() {
                    match download_static_content(&external_url).await {
                        Ok(local_path) => {
                            let relative_path = Path::new(&local_path)
                                .strip_prefix("downloads/pages")
                                .unwrap_or_else(|_| Path::new(&local_path))
                                .to_str()
                                .unwrap()
                                .to_string();
                            replacements.push((original_link, relative_path));
                        }
                        Err(e) => eprintln!("Failed to download asset {}: {}", external_url, e),
                    }
                }

                let updated_html = replace_external_links(&html, &replacements);

                fs::write(path, updated_html.as_bytes()).await?;

                for (original_link, css_url) in external_assets
                    .iter()
                    .filter(|(link, _)| link.ends_with(".css"))
                {
                    if let Ok(css_content) = download_page(css_url).await {
                        let modified_css = modify_and_download_css(css_url, &css_content).await?;
                        let css_local_path = format!(
                            "downloads/static/{}",
                            css_url.path().trim_start_matches('/')
                        );
                        fs::write(&css_local_path, modified_css).await?;

                        replacements.push((original_link, css_local_path));
                    }
                }

                for link in internal_links {
                    if !visited_pages.contains(&link) {
                        to_visit.push(link.clone());
                    }
                }

                visited_pages.insert(url);
            }
            Err(e) => eprintln!("Failed to download {}: {}", url, e),
        }

        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Err(e) = crawl_website(&args.url).await {
        eprintln!("An error occurred: {}", e);
    }
}
