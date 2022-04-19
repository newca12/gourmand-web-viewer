mod cli;
fn main() {
    let opt = cli::parse_cli();
    cli::run_cli(opt);
}
