use anyhow::{Context, Result};
use serde::Deserialize;
use std::{collections::HashSet, fs::File, path::Path};

#[derive(Deserialize, Debug)]
pub struct Follower {
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct FollowerWrapper {
    pub string_list_data: Vec<Follower>,
}

pub fn parse_followers(path: &Path) -> Result<HashSet<String>> {
    let f = File::open(path)?;

    let r = std::io::BufReader::new(f);

    let follower_list: Vec<FollowerWrapper> =
        serde_json::from_reader(r).context("parse follower list with serde")?;

    let follower_list: HashSet<String> = follower_list
        .into_iter()
        .map(|f| f.string_list_data[0].value.clone())
        .collect();

    Ok(follower_list)
}
