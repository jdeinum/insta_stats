use crate::{cli::Cli, followers::parse_followers, following::parse_following};
use anyhow::{Context, Result};
use clap::Parser;
use itertools::Itertools;
use std::path::Path;

pub fn run() -> Result<()> {
    // parse cli
    let cli = Cli::parse();

    // parse the followers into a list
    let followers = parse_followers(Path::new(&cli.followers_file)).context("parse followers")?;

    // parse the following into a list
    let following = parse_following(Path::new(&cli.following_file)).context("parse following")?;

    // compute mutal
    if cli.mutal {
        println!("Mutual: ");
        following
            .intersection(&followers)
            .into_iter()
            .sorted()
            .for_each(|f| println!("{f}"));
        println!("\n\n");
    }

    // compute you not following back
    if cli.not_following_them {
        println!("Not following them back: ");
        followers
            .difference(&following)
            .into_iter()
            .sorted()
            .for_each(|f| println!("{f}"));
        println!("\n\n");
    }

    // compute them not following back
    if cli.them_not_you {
        println!("They don't follow you back: ");
        following
            .difference(&followers)
            .into_iter()
            .sorted()
            .for_each(|f| println!("{f}"));
        println!("\n\n");
    }

    Ok(())
}
