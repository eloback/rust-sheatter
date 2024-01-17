use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct SheatterArgs {
    /// topic to list sheets
    pub topic: String,
}
