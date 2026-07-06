use clap::Args;

#[derive(Args)]
pub struct ExportArgs {
    #[arg(long, short, help = "Season's name")]
    season: String,
    #[arg(long, short, default_value_t = 1, help = "Last N games to export, 0 to get only the summary")]
    count: u8,
}