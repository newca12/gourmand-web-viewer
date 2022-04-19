use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;
use std::path::PathBuf;

use gourmand_web_viewer_cli::check;
use gourmand_web_viewer_cli::load_xml;
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
        let (categories, cuisines, recipes) = load_xml(false);
        match opt.command.as_deref() {
            Some("generate") => {
                let file = File::create("gourmand-web-viewer-cli/src/data/recipes.json")
                    .expect("Cannot create output file");
                serde_json::to_writer(file, &recipes).expect("Cannot write to output file");
                let file = File::create("gourmand-web-viewer-cli/src/data/categories.json")
                    .expect("Cannot create output file");
                serde_json::to_writer(file, &categories).expect("Cannot write to output file");
                let file = File::create("gourmand-web-viewer-cli/src/data/cuisines.json")
                    .expect("Cannot create output file");
                serde_json::to_writer(file, &cuisines).expect("Cannot write to output file");
            }
            Some("list") => {
                if opt.output.is_some() {
                    let file =
                        File::create(opt.output.unwrap()).expect("Cannot create output file");
                    let mut file = LineWriter::new(file);
                    for (key_variable, _) in recipes.iter() {
                        file.write_all(key_variable.as_bytes())
                            .expect("Cannot write output file");
                        file.write_all(b"\n").expect("Cannot write output file");
                    }
                } else {
                    for (key_variable, _) in recipes.iter() {
                        println!("{}", key_variable);
                    }
                }
            }
            Some("debug") => {
                let (_, _, recipes) = load_xml(true);
                check(recipes);
            }
            _ => println!("unkown command"),
        }
    }
}
