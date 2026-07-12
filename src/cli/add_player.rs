use clap::Args;

#[derive(Args)]
pub struct AddPlayerArgs {
    #[arg(help = "Season's name")]
    season: String,
    #[arg(help = "Player's name")]
    name: String
}