use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct Opts {
    #[structopt(short, long)]
    pub url: String,
}

impl Opts {
    pub fn read() -> Self {
        let opts = Opts::from_args();
        opts
    }
}
