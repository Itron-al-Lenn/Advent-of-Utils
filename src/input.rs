use reqwest::blocking::Client;
use std::{fmt::Display, sync::Arc};

use crate::error::InputError;

const USER_AGENT: &str =
    "Advent of Utils by Itron-al-Lenn found on github.com/Itron-al-Lenn/Advent-of-Utils";
const AOC_BASE_URL: &str = "https://adventofcode.com";

#[derive(Debug, Clone)]
pub struct SessionToken(String);

impl SessionToken {
    pub fn new() -> Result<Self, InputError> {
        match std::env::var("AOC_SESSION") {
            Ok(token) => Ok(Self(token)),
            Err(e) => Err(InputError::MissingToken {
                reason: format!("AOC_SESSION environment variable not set: {}", e),
            }),
        }
    }
}

impl Display for SessionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for SessionToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

fn create_client() -> Result<Client, InputError> {
    let cookie = reqwest::cookie::Jar::default();
    cookie.add_cookie_str(
        &format!("session={}", SessionToken::new()?),
        &AOC_BASE_URL.parse::<reqwest::Url>().unwrap(),
    );

    Ok(Client::builder()
        .cookie_provider(Arc::new(cookie))
        .user_agent(USER_AGENT)
        .build()
        .expect("Failed to create HTTP client"))
}

pub fn fetch_input(year: i32, day: u8) -> Result<String, InputError> {
    let url = format!("{}/{}/day/{}/input", AOC_BASE_URL, year, day);

    create_client()?
        .get(&url)
        .send()
        .map_err(|e| InputError::FetchFailed {
            year,
            day,
            reason: "Network request failed".to_string(),
            source: Some(e),
        })?
        .error_for_status()
        .map_err(|e| InputError::FetchFailed {
            year,
            day,
            reason: "Server returned error status".to_string(),
            source: Some(e),
        })?
        .text()
        .map_err(|e| InputError::FetchFailed {
            year,
            day,
            reason: "Failed to read response text".to_string(),
            source: Some(e),
        })
}
