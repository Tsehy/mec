use clap::Args;

#[derive(Args)]
pub struct ExportArgs {
    #[arg(help = "Season's name")]
    season: String,
    #[arg(long, short = 'n', default_value_t = 1, help = "Last N games to export, 0 to get only the summary")]
    count: u8,
    #[arg(long, short, default_value_t = false, help = "Make game files inline")]
    inline: bool,
    #[arg(long, short, default_value_t = 0x8D0404, help = "Color of the margin (HEX)")]
    color: u32,
}

impl ExportArgs {
    pub fn season(&self) -> &str {
        &self.season
    }

    pub fn count(&self) -> u8 {
        self.count
    }
    
    pub fn inline(&self) -> bool {
        self.inline
    }
    
    pub fn color(&self) -> u32 {
        self.color
    }
}