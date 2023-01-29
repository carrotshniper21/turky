use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

use std::io;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct ShowResult {
    id: String,
    title: String,
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sources {
    url: String,
    quality: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subtitles {
    url: String,
    lang: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
    id: String,
    title: String,
    number: usize,
    season: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stream {
    sources: Vec<Sources>,
    subtitles: Vec<Subtitles>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Shows {
    episodes: Vec<Results>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Json {
    results: Vec<ShowResult>,
}

pub fn main(input: String) -> Result<(Subtitles, Vec<String>), Box<dyn std::error::Error>> {
    let shows = get_shows(&input)?;
    if shows.results.len() == 0 {
        println!("[*] No Results Found");
    }

    for (i, show) in shows.results.iter().enumerate() {
        println!(
            "[{}]: ({}), ({})",
            i + 1,
            show.title,
            show.r#type
        );
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let show_index: usize = input.trim().parse::<usize>().unwrap() - 1;
    let show_id: String = shows.results[show_index].id.clone();
    let chosen_id = get_info(&show_id).unwrap();

    let seasons = chosen_id.episodes.iter().map(|ep| ep.season).max().unwrap();
    input.clear();

    print!("Choose a season 1-{}: ", seasons);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input).unwrap();
    let season_number: usize = input.trim().parse().unwrap();

    input.clear();
    let episodes = chosen_id
        .episodes
        .iter()
        .filter(|ep| ep.season == season_number)
        .map(|ep| ep.number)
        .max()
        .unwrap();
    print!("Choose a episode 1-{}: ", episodes);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input).unwrap();
    let episode_number: usize = input.trim().parse().unwrap();

    let chosen_episode = chosen_id
        .episodes
        .iter()
        .find(|ep| ep.season == season_number && ep.number == episode_number)
        .unwrap();

    let stream = get_stream(&chosen_episode.id, &show_id).unwrap();
    let _url: Vec<&str> = stream
        .sources
        .iter()
        .map(|source| source.url.as_str())
        .collect();
    let subtitles = stream.subtitles;

    let chosen_lang = subtitles
        .into_iter()
        .find(|sub| sub.lang.starts_with("English"))
        .unwrap();
    
    Ok((chosen_lang, _url.into_iter().map(|show_url| show_url.to_string()).collect()))
}

pub fn get_shows(query: &str) -> Result<Json, Box<dyn std::error::Error>> {
    let url = format!("http://api.consumet.org/movies/flixhq/{}", query);
    let resp: Value = reqwest::blocking::get(&url)?.json()?;
    let shows: Json = serde_json::from_value(resp)?;
    Ok(shows)
}

pub fn get_info(id: &str) -> Result<Shows, Box<dyn std::error::Error>> {
    let url = format!("https://api.consumet.org/movies/flixhq/info?id={}", id);
    let resp: Value = reqwest::blocking::get(&url)?.json()?;
    let shows: Shows = serde_json::from_value(resp)?;
    Ok(shows)
}

pub fn get_stream(episode_id: &str, show_id: &str) -> Result<Stream, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.consumet.org/movies/flixhq/watch?episodeId={}&mediaId={}",
        episode_id, show_id
    );
    let resp: Value = reqwest::blocking::get(&url)?.json()?;
    let stream: Stream = serde_json::from_value(resp)?;
    Ok(stream)
}
