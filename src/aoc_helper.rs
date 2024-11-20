struct AocHelper {
    year: u8,
    day: u8,
    token: String,
}

impl AocHelper {
    pub fn new(u8: year, u8: day) -> AocHelper {
        let token = std::env::var("AOC_SESSION").expect("No AOC_SESSION environement variable set");
        Self { day, year, token }
    }
    fn get_input(&self) -> String {
        let file_location = format!("input/{}_{}", self.day, self.year);
        match std::fs::read_to_string(file_location) {
            Ok(input) => input,
            Err(_) => {
                self.fetch_input();
                self.get_input()
            }
        }
    }
    fn fetch_input(&self) {
        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            self.year, self.day
        );
        let cookie = reqwest::cookie::Jar::default();
        cookie.add_cookie_str(
            &format!("session={}", self.token),
            &url.parse::<reqwest::Url>().unwrap(),
        );
        let client = reqwest::blocking::Client::builder()
            .cookie_provider(Arc::new(cookie))
            .user_agent("Advent of Utils by Itron-al-Lenn found on ")
            .build()?;
    }
}
