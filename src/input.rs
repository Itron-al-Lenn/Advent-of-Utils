use lazy_static::lazy_static;
use reqwest::blocking::Client;
use std::{fmt::Display, sync::Arc};

const USER_AGENT: &str =
    "Advent of Utils by Itron-al-Lenn found on github.com/Itron-al-Lenn/Advent-of-Utils";
const AOC_BASE_URL: &str = "https://adventofcode.com";

#[derive(Debug, Clone)]
pub struct SessionToken(String);

impl Default for SessionToken {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionToken {
    pub fn new() -> Self {
        Self(std::env::var("AOC_SESSION").expect("No AOC_SESSION environment variable set"))
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

lazy_static! {
    static ref HTTP_CLIENT: Client = create_client();
}

fn create_client() -> Client {
    let cookie = reqwest::cookie::Jar::default();
    cookie.add_cookie_str(
        &format!("session={}", SessionToken::new()),
        &AOC_BASE_URL.parse::<reqwest::Url>().unwrap(),
    );

    Client::builder()
        .cookie_provider(Arc::new(cookie))
        .user_agent(USER_AGENT)
        .build()
        .expect("Failed to create HTTP client")
}

#[derive(Debug)]
pub enum InputError {
    Network(reqwest::Error),
    InvalidResponse(String),
}

impl std::error::Error for InputError {}

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Network(e) => write!(f, "Network error: {}", e),
            Self::InvalidResponse(e) => write!(f, "Invalid response: {}", e),
        }
    }
}

pub fn fetch_input(year: i32, day: u8) -> Result<String, InputError> {
    let url = format!("{}/{}/day/{}/input", AOC_BASE_URL, year, day);

    HTTP_CLIENT
        .get(&url)
        .send()
        .map_err(InputError::Network)?
        .error_for_status()
        .map_err(InputError::Network)?
        .text()
        .map_err(|e| InputError::InvalidResponse(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_token_display() {
        let token = SessionToken("test123".to_string());
        assert_eq!(token.to_string(), "test123");
    }

    #[test]
    fn test_session_token_from_string() {
        let token = SessionToken::from("test123".to_string());
        assert_eq!(token.0, "test123");
    }
}
