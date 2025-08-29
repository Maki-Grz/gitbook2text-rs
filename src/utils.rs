use pulldown_cmark::{Event, Parser};
use regex::Regex;
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

pub fn txt_sanitize(txt: &str) -> String {
    let mut result = String::from(txt);

    let re_code = Regex::new(r#"\{%\s*code[^}]*title\s*=\s*"([^"]+)"[^}]*%}(.*?)\{%\s*endcode\s*%\}"#).unwrap();
    result = re_code
        .replace_all(&result, |caps: &regex::Captures| {
            format!("{} {}", &caps[1], &caps[2])
        })
        .to_string();

    let re_code_no_title = Regex::new(r#"\{%\s*code[^}]*%}(.*?)\{%\s*endcode\s*%\}"#).unwrap();
    result = re_code_no_title
        .replace_all(&result, |caps: &regex::Captures| {
            caps[1].to_string()
        })
        .to_string();

    let re_title = Regex::new(r#"\{%\s*[^}]*title\s*=\s*"([^"]+)"[^}]*%\}"#).unwrap();
    result = re_title.replace_all(&result, "$1").to_string();

    let re_generic = Regex::new(r#"\{%\s*[^}]*%\}"#).unwrap();
    result = re_generic.replace_all(&result, "").to_string();

    let re_dash = Regex::new(r"-").unwrap();
    result = re_dash.replace_all(&result, " ").to_string();

    let re_space = Regex::new(r"\s+").unwrap();
    result = re_space.replace_all(&result, " ").to_string();

    result.trim().to_string()
}

pub async fn save_text(url: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename = url_to_filename(url) + ".txt";
    fs::write(format!("data/txt/{}", filename), content).await?;
    Ok(())
}

