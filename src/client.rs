use crate::Options;

use std::{error::Error, process};

use fancy_regex::{Matches, Regex};
use owo_colors::{colors::Red, OwoColorize};

const PLAYER_FN: &'static str = "mpv";
const BASE_API_URL: &'static str = "https://gogoanime.vc";
// const LOGFILE: &'static str = "${XDG_CACHE_HOME:-$HOME/.cache}/ani-hsts";

pub struct Client {
    title: String,
    download: bool,
    cont: bool,
    quality: Option<u32>,
}

impl Client {
    pub fn new(options: Options) -> Self {
        let title = options.title.unwrap_or("".to_string());
        Self {
            title,
            download: options.download,
            cont: options.cont,
            quality: options.quality,
        }
    }

    // Gets anime names along with their ids
    pub async fn search_anime<'r, 't>(
        &self,
        title: &str,
    ) -> Result<Option<Matches<'r, 't>>, Box<dyn Error>> {
        let title = title.replace(' ', "-");
        let url = format!("{}//search.html?keyword={}", BASE_API_URL, title);
        let re =
            Regex::new(r#"s_^[[:space:]]*<a href="/category/([^"]*)" title="([^"]*)".*_\1_p"#)?;

        let response = reqwest::get(url).await?;
        let data = response.text().await?;
        let matches = re.captures_iter(&data);

        println!("{:?}", matches.count());

        // Ok(matches)
        Ok(None)
    }

    // Get available episodes from an id
    pub async fn search_episodes<'r, 't>(
        &self,
        id: &str,
    ) -> Result<Option<Matches<'r, 't>>, Box<dyn Error>> {
        let url = format!("{}/category/{}", BASE_API_URL, id);
        let re = Regex::new(
            r#"/^[[:space:]]*<a href="\#" class="active" ep_start/{
		s/.* '\''([0-9]*)'\'' ep_end = '\''([0-9]*)'\''.*/\2/p
		q
		}"#,
        )?;

        let response = reqwest::get(url).await?;
        let data = response.text().await?;
        let matches = re.find_iter(&data);

        Ok(None)
    }

    pub async fn get_embedded_video_link<'t>(&self, title: &str, id: &str) -> Option<&'t str> {
        None
    }

    pub async fn get_video_quality<'t>(&self, title: &str, id: &str) -> Option<&'t str> {
        None
    }

    pub async fn get_links<'t>(&self, title: &str) -> Option<&'t str> {
        None
    }

    fn eprintln(&self, err: &str) {
        eprintln!("{}", err.fg::<Red>());
    }

    fn die(&self) {
        self.eprintln("test error");
        process::exit(1);
    }
}
