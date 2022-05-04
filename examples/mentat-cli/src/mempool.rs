use clap::Parser;

#[derive(Parser, Clone)]
pub(crate) struct MempoolOpts {
    #[clap(short, long)]
    pub(crate) transaction: Option<String>,
}
