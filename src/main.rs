extern crate clap;

use std::env;

use clap::{App, Arg, SubCommand};

mod io;
mod r2k;

fn main() {
    let mut args: env::Args = env::args();
    args.next();
    let args = args;

    let cmds = clap();
    let file = io::get_file();
    if let None = file {
        return;
    }

    let map = r2k::get_dict();
    for arg in args {
        let tmp = r2k::do_work(&map, &arg);
        println!("\"{}\"\t=>\t{:?}", arg, tmp);
    }
    println!();
}

fn clap() -> App<'static, 'static> {
    App::new("Japanese Command-line Dictionary")
        .version("0.0.1")
        .author("https://github.com/siiky")
        .about("Dictionary to keep track of learned/seen words and Romaji to Kana converter")
        .subcommand(SubCommand::with_name("add")
            .about("Add a word to the dictionary.")
            .arg(Arg::with_name("kana").short("k").takes_value(true))
            .arg(Arg::with_name("kanji").short("K").takes_value(true))
            .arg(Arg::with_name("meaning").short("m").takes_value(true)))
        .subcommand(SubCommand::with_name("search"))
        .subcommand(SubCommand::with_name("convert"))
        .arg(Arg::with_name("romaji").short("r").help("Converts romaji to kana").takes_value(true))
}
