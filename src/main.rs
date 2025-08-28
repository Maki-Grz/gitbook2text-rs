mod utils;

use crate::utils::{download_page, markdown_to_text, save_markdown, save_text};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::collections::HashSet;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("links.txt");

    /*todo: get markdown from gitbook uri if possible */

    let mut urls: HashSet<String> = content?
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    let new_urls_with_md = urls.drain()
        .map(|mut u| {
            u.push_str(".md");
            u
        })
        .collect();

    urls = new_urls_with_md;

    let mut futures = FuturesUnordered::new();

    for url in urls {
        let url_clone = url.clone();
        futures.push(async move {
            let md_content = download_page(&url_clone).await?;
            save_markdown(&url_clone, &md_content).await?;

            let text_content = markdown_to_text(&md_content);
            save_text(&url_clone, &text_content).await?;

            /*todo: Markedown sanitizer (remove html & markedown tags) */

            Ok::<(), Box<dyn std::error::Error>>(())
        });
    }

    while let Some(result) = futures.next().await {
        match result {
            Ok(_) => println!("Page saved"),
            Err(e) => eprintln!("Erreur: {:?}", e),
        }
    }


    Ok(())
}
