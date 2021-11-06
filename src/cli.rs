use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;
use std::path::PathBuf;

use gourmand_web_viewer::check;
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
    #[structopt(long)]
    pub cli: bool,
    #[structopt(short, long)]
    pub command: Option<String>,
    #[structopt(short, long, parse(from_os_str))]
    pub output: Option<PathBuf>,
}

pub fn parse_cli() -> Opt {
    Opt::from_args()
}

pub fn run_cli(opt: Opt) {
    if opt.command.is_some() {
        let list = gourmand_web_viewer::load(false);
        match opt.command.as_deref() {
            Some("list") => {
                if opt.output.is_some() {
                    let file =
                        File::create(opt.output.unwrap()).expect("Cannot create output file");
                    let mut file = LineWriter::new(file);
                    for (key_variable, _) in list.iter() {
                        file.write_all(key_variable.as_bytes())
                            .expect("Cannot write output file");
                        file.write_all(b"\n").expect("Cannot write output file");
                    }
                } else {
                    for (key_variable, _) in list.iter() {
                        println!("{}", key_variable);
                    }
                }
            }
            Some("debug") => {
                let recipes = gourmand_web_viewer::load(true);
                check(recipes);
            }
            _ => println!("unkown command"),
        }
    }
}
