use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ani-cli", about = "An anime mpv client.")]
pub struct Options {
    #[structopt(short, long, help = "download episode")]
    pub download: bool,
    #[structopt(short = "H", long = "continue", help = "continue where you left off")]
    pub cont: bool,
    #[structopt(short, long, help = "set video quality (best/worst/360/480/720/..)")]
    pub quality: Option<u32>,
    #[structopt(help = "anime title")]
    pub title: Option<String>,
}
