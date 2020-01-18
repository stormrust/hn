use colored::*;
use reqwest::Error;
use scraper::{ElementRef, Html, Selector};
use std::result::Result;

fn main() {
    match get_body() {
        Ok(body) => {
            for element in Html::parse_document(&body).select(&selector()) {
                parse_story(element);
                parse_link(element);
            }
        }
        Err(_) => eprintln!("Error fetching Hacker News data."),
    }
}

fn get_body() -> Result<String, Error> {
    reqwest::blocking::get("https://news.ycombinator.com/")?
        .text()
}

fn selector() -> Selector {
    Selector::parse("a.storylink").unwrap()
}

fn parse_link(element: ElementRef) {
    if let Some(link) = element.value().attr("href") {
        let mut link = link.to_owned();
        if link.starts_with("item?") {
            link = format!("https://news.ycombinator.com/{}", link);
        }
        println!("\t{}", link);
    }
}

fn parse_story(element: ElementRef) {
    println!("{}", element.inner_html().cyan());
}
