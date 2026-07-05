use clap::Args;

#[derive(Args)]
pub struct ExportArgs {
    #[arg(long, short)]
    season: String,
    #[arg(long, short, default_value_t = 1, )]
    count: u8,
}