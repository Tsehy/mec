use clap::Args;

#[derive(Args)]
pub struct DisplayArgs {
    #[arg(long, short)]
    season: String,
    #[arg(default_value_t = 0)]
    count: u8,
}