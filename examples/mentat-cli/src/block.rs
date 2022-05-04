use clap::Parser;

#[derive(Parser, Clone)]
pub(crate) struct BlockOpts {
    #[clap(short, long)]
    pub(crate) transaction: Option<String>,
}
