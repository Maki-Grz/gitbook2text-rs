use pulldown_cmark::{Event, Parser};
use tokio::fs;

pub async fn download_page(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?;
    let text = resp.text().await?;
    Ok(text)
}

pub async fn save_markdown(url: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename = url_to_filename(url) + ".md";
    fs::write(format!("data/md/{}", filename), content).await?;
    Ok(())
}


pub fn url_to_filename(url: &str) -> String {
    url.replace("/", "_").replace(":", "_")
}


pub fn markdown_to_text(md: &str) -> String {
    let parser = Parser::new(md);
    let mut text = String::new();

    for event in parser {
        match event {
            Event::Text(t) => text.push_str(&t),
            Event::Code(t) => text.push_str(&t),
            Event::SoftBreak | Event::HardBreak => text.push('\n'),
            _ => {}
        }
    }

    text
}

pub async fn save_text(url: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename = url_to_filename(url) + ".txt";
    fs::write(format!("data/txt/{}", filename), content).await?;
    Ok(())
}

