use std::{fmt::Display, sync::Arc};

pub struct SessionToken(pub String);

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

pub fn fetch_input(year: i32, day: u8) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let cookie = reqwest::cookie::Jar::default();
    cookie.add_cookie_str(
        &format!("session={}", SessionToken::new()),
        &url.parse::<reqwest::Url>().unwrap(),
    );
    let client = reqwest::blocking::Client::builder()
        .cookie_provider(Arc::new(cookie))
        .user_agent(
            "Advent of Utils by Itron-al-Lenn found on github.com/Itron-al-Lenn/Advent-of-Utils",
        )
        .build()
        .expect("Failed building the client");

    Ok(client.get(url).send()?.text()?)
}
