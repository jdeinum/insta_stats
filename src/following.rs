use anyhow::{Context, Result};
use serde::Deserialize;
use std::{collections::HashSet, fs::File, path::Path};

#[derive(Deserialize, Debug)]
pub struct Following {
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct FollowingWrapper {
    pub string_list_data: Vec<Following>,
}

#[derive(Deserialize, Debug)]
pub struct FollowingFile {
    pub relationships_following: Vec<FollowingWrapper>,
}

pub fn parse_following(path: &Path) -> Result<HashSet<String>> {
    let f = File::open(path)?;

    let r = std::io::BufReader::new(f);

    let following_list: FollowingFile =
        serde_json::from_reader(r).context("parse follower list with serde")?;

    let follower_list: HashSet<String> = following_list
        .relationships_following
        .into_iter()
        .map(|f| f.string_list_data[0].value.clone())
        .collect();

    Ok(follower_list)
}
