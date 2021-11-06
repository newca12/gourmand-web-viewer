mod cli;
mod gui;
fn main() {
    let opt = cli::parse_cli();
    match opt.cli {
        false => {
            gui::run_gui();
        }
        true => {
            cli::run_cli(opt);
        }
    };
}
