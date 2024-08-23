extern crate reqwest;
extern crate scraper;

use reqwest::Client;
use scraper::error::SelectorErrorKind;
use scraper::{Html, Selector};
use std::error::Error;

#[derive(Debug)]
pub enum CustomError {
    ReqwestError(reqwest::Error),
    ScraperError(SelectorErrorKind<'static>),
}

//Implement conversions from specific error types to CustomError
impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> Self {
        CustomError::ReqwestError(err)
    }
}

impl From<SelectorErrorKind<'static>> for CustomError {
    fn from(err: SelectorErrorKind<'static>) -> Self {
        CustomError::ScraperError(err)
    }
}

// Function to get the data from the URL
pub async fn get_curl_data() -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get("https://hianime.to/az-list")
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

// Function to extract the last page number from the response
pub async fn get_last_page_no_of_atoz_list() -> Result<u16, Box<dyn Error>> {
    let response = get_curl_data().await?;
    let document = Html::parse_document(&response);

    // Selector for the last page link
    let nav_selector = Selector::parse("#main-wrapper > div > div.page-az-wrap > section > div.tab-content > div > div.pre-pagination.mt-5.mb-5 > nav > ul > li:last-child a")?;

    // Find the last page link
    if let Some(last_page_element) = document.select(&nav_selector).last() {
        if let Some(href) = last_page_element.value().attr("href") {
            if let Some(page_str) = href.split('=').last() {
                if let Ok(page_number) = page_str.parse::<u16>() {
                    return Ok(page_number);
                }
            }
        }
    }

    Ok(212)
}
