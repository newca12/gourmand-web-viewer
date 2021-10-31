use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long)]
    cli: bool,
}

pub fn parse_cli() -> bool {
    let opt = Opt::from_args();
    opt.cli
}

pub fn run_cli() {
    println!("Not yet implemented");
}
