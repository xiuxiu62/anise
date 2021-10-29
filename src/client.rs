use crate::Options;

use std::{error::Error, process, time::Duration};

use fancy_regex::{Matches, Regex};
use owo_colors::{colors::Red, OwoColorize};
use scraper::{Html, Selector};
use ureq::{Agent, AgentBuilder};

const PLAYER_FN: &'static str = "mpv";
const BASE_API_URL: &'static str = "https://gogoanime.vc";
// const LOGFILE: &'static str = "${XDG_CACHE_HOME:-$HOME/.cache}/ani-hsts";

pub struct Client {
    agent: Agent,
    title: String,
    download: bool,
    cont: bool,
    quality: Option<u32>,
}

impl Client {
    pub fn new(options: Options) -> Self {
        let title = options.title.unwrap_or("".to_string());
        let agent = AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();

        Self {
            agent,
            title,
            download: options.download,
            cont: options.cont,
            quality: options.quality,
        }
    }

    // Gets anime names along with their ids
    pub fn search_anime<'a>(&self, title: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let title = title.replace(' ', "-");
        let url = format!("{BASE_API_URL}//search.html?keyword={title}");
        let a_selector = Selector::parse("a").unwrap();

        let response = self.agent.get(&url).call()?;
        let doc = Html::parse_document(&response.into_string()?);
        let titles = doc
            .select(&a_selector)
            .filter(|elem| elem.inner_html().contains(&title))
            .fold(Vec::new(), |mut acc, elem| {
                if let Some(title) = elem.value().attr("title") {
                    acc.push(title.to_string());
                }
                acc
            });

        Ok(titles)
    }

    // Get available episodes from an id
    pub fn search_episodes<'r, 't>(
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

        let response = self.agent.get(&url).call()?;
        let data = response.into_string()?;
        let matches = re.captures_iter(&data);

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
