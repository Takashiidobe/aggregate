use anyhow::Result;
use clap::Parser;
use glob::glob;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use titlecase::titlecase;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    glob_str: String,
    #[arg(short, long)]
    file: String,
}

fn main() -> Result<()> {
    let Args { glob_str, file } = Args::parse();
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(file)?;
    let mut s = String::new();
    let path = env::current_dir()?;
    let path = path.display().to_string();
    let pwd = path.split('/').last().unwrap();

    s.push_str("---\n");
    s.push_str(&format!(r#"title: "{}""#, titlecase(pwd)));
    s.push_str("\n---\n\n");
    s.push_str("# ");
    s.push_str(&titlecase(pwd));
    s.push_str("\n\n");

    for f in glob(&glob_str)? {
        let f = f?;
        let file_name = f.file_name();
        let file_name = file_name.unwrap().to_str().unwrap();
        if file_name.starts_with('_') {
            continue;
        }
        let (file_start, _rest) = file_name.split_once('.').unwrap();

        let f = f.display();
        s.push_str(&format!(
            "- [{}]({})\n",
            titlecase(file_start).replace('-', " "),
            f
        ));
    }
    file.write_all(s.as_bytes())?;

    Ok(())
}
