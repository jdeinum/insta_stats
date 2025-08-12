use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// The path to the followers file downloaded from insta
    #[arg(long)]
    pub followers_file: String,

    /// The path to the following file downloaded from insta
    #[arg(long)]
    pub following_file: String,

    /// Show mutual followers
    #[arg(short, long)]
    pub mutal: bool,

    /// Show people that you follow but not vice versa
    #[arg(short, long)]
    pub them_not_you: bool,

    /// Show people follow you but not vice versa
    #[arg(short, long)]
    pub not_following_them: bool,
}
