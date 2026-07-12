use clap::Args;

#[derive(Args)]
pub struct RemoveGameArgs {
    #[arg(help = "Season's name")]
    season: String,
    #[arg(default_value_t = 1, help = "Last N game to remove")]
    count: u32,
}