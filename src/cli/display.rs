use clap::Args;

#[derive(Args)]
pub struct DisplayArgs {
    #[arg(long, short, help = "Season's name")]
    season: String,
    #[arg(default_value_t = 0, help = "Last N games to display, 0 to display all")]
    count: u8,
}