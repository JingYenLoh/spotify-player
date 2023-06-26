//! # lyric_finder
//!
//! This crate provides a [`Client`](Client) struct for retrieving a song's lyric.
//!
//! It ultilizes the [Genius](https://genius.com) website and its APIs to get lyric data.
//!
//! ## Example
//!
//! ```rust
//! # use anyhow::Result;
//! #
//! # async fn run() -> Result<()> {
//! let client =  lyric_finder::Client::new();
//! let result = client.get_lyric("shape of you").await?;
//! match result {
//!     lyric_finder::LyricResult::Some {
//!         track,
//!         artists,
//!         lyric,
//!     } => {
//!         println!("{} by {}'s lyric:\n{}", track, artists, lyric);
//!     }
//!     lyric_finder::LyricResult::None => {
//!         println!("lyric not found!");
//!     }
//! }
//! # Ok(())
//! # }
//! ```

use crate::search::LyricsEntity;

pub struct Client {
    http: reqwest::Client,
}

#[derive(Debug)]
pub enum LyricResult {
    Some { lyric: String },
    None,
}

impl Client {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
        }
    }

    /// Construct a client reusing an exisiting http client
    pub fn from_http_client(http: &reqwest::Client) -> Self {
        Self { http: http.clone() }
    }

    /// Get the lyric of a song satisfying a given `query`.
    pub async fn get_lyric(&self, id: &str) -> anyhow::Result<LyricResult> {
        let url = format!("https://api.lyricstify.vercel.app/v1/lyrics/{id}");

        log::debug!("fetching for {}", &id);

        let body = self
            .http
            .get(url)
            .send()
            .await?
            .json::<LyricsEntity>()
            .await?;

        let lyric = body
            .lyrics
            .lines
            .iter()
            .fold(String::new(), |mut acc, line| {
                acc.push_str(&line.words);
                acc.push('\n');
                acc
            });

        Ok(LyricResult::Some { lyric })
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

mod search {
    use serde::Deserialize;
    #[derive(Debug, Deserialize)]
    pub struct LyricsEntity {
        pub lyrics: Lyrics,
    }

    #[derive(Debug, Deserialize)]
    pub struct Lyrics {
        #[serde(rename = "syncType")]
        pub sync_type: String,
        pub lines: Vec<Line>,
        pub language: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Line {
        #[serde(rename = "startTimeMs")]
        pub start_time_ms: u64,
        pub words: String,
    }
}
