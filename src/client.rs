use crate::{error::AniseResult, show::Show, Options};

use std::{fmt::Debug, process, time::Duration};

use owo_colors::{colors::Red, OwoColorize};
use regex::Regex;
use scraper::{html, ElementRef, Html, Selector};
use ureq::{Agent, AgentBuilder, Response};

const PLAYER_FN: &str = "mpv";
const BASE_API_URL: &str = "https://gogoanime.vc";
// const LOGFILE: &str = "${XDG_CACHE_HOME:-$HOME/.cache}/ani-hsts";

pub struct Client {
    agent: Agent,
    title: String,
    download: bool,
    cont: bool,
    quality: Option<u32>,
}

type Shows = Vec<Show>;

impl Client {
    pub fn new(options: Options) -> Self {
        let title = options.title.unwrap_or_else(String::new);
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
    pub fn search_anime(&self, title: &str) -> AniseResult<Shows> {
        let title = title.replace(' ', "-");
        let url = format!("{BASE_API_URL}//search.html?keyword={title}");
        let response = self.agent.get(&url).call()?;
        let parse_method = &|select: html::Select| {
            select
                .filter(|elem| elem.inner_html().contains(&title))
                .fold(Vec::new(), |mut acc: Vec<String>, elem: ElementRef| {
                    if let Some(name) = elem.value().attr("title") {
                        acc.push(name.to_string());
                    }
                    acc
                })
        };

        let names = parse_response_by_selector(response, "a", parse_method)?;
        let ids = names
            .iter()
            .map(|name| strip_title(&name))
            .collect::<AniseResult<Vec<String>>>()?;
        let shows = names
            .into_iter()
            .zip(ids)
            .fold(Vec::new(), |mut acc, show| {
                acc.push(Show::from(show));
                acc
            });

        Ok(shows)
    }

    // Get available episodes from a title
    pub fn search_episodes(&self, title: &str) -> AniseResult<()> {
        let url = format!("{}/category/{}", BASE_API_URL, title);
        let _response = self.agent.get(&url).call()?;

        Ok(())
    }

    pub async fn get_embedded_video_link<'t>(&self, _title: &str, _id: &str) -> Option<&'t str> {
        None
    }

    pub async fn get_video_quality<'t>(&self, _title: &str, _id: &str) -> Option<&'t str> {
        None
    }

    pub async fn get_links<'t>(&self, _title: &str) -> Option<&'t str> {
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

// Queries an http response by tag
fn parse_response_by_selector<T>(
    response: Response,
    tag: &str,
    parse_method: &dyn Fn(html::Select) -> T,
) -> AniseResult<T>
where
    T: Sized + Debug,
{
    let doc = Html::parse_document(&response.into_string()?);
    let selector = Selector::parse(tag).unwrap();
    let result = parse_method(doc.select(&selector));

    Ok(result)
}

fn strip_title(title: &str) -> AniseResult<String> {
    let title = title.to_string().to_lowercase();
    let re_non_alpha_numeric = Regex::new(r"/[^a-z0-9]/g")?;

    re_non_alpha_numeric.replace_all(&title, "");
    Ok(title)
}
