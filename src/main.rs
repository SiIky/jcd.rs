#[macro_use]
extern crate clap;

// use std::env;

use clap::{App, Arg, ArgMatches, SubCommand};

mod io;
mod r2k;

fn main() {
    // let mut args: env::Args = env::args();
    // args.next();
    // let args = args;
    // for arg in args { let tmp = r2k::do_work(&map, &arg); println!("\"{}\"\t=>\t{:?}", arg, tmp); }
    // println!();

    // let file = io::get_file();
    // if let None = file { return; }

    let _map = r2k::get_dict();

    println!("before");
    let matches = clap();
    let subcmd = match matches.subcommand_name() {
        None => panic!("No Subcommand was used!"),
        Some(sc) => sc,
    };
    let sub_matches = match matches.subcommand_matches(subcmd) {
        None => panic!("No matches for subcommand!"),
        Some(sm) => sm,
    };
    println!("after");

    match subcmd {
        "add" => handle_add(),
        "search" => handle_search(),
        "convert" => handle_convert(),
        _ => panic!("Command not recognized!"),
    }
}

fn handle_add() {
    unimplemented!();
}

fn handle_search() {
    unimplemented!();
}

fn handle_convert() {
    unimplemented!();
}

///
/// Usage: (This comment will be used to describe the expected behavior and the program must fit this
///        description, not the other way around)
///
/// * `add`: Add a word to the dictionary.
///     * `-r`: Convert a word and add it to the dictionary. (Use auto detection);
///     * `-h`: Convert a word to hiragana and add it to the dictionary. (Don't use auto detection);
///     * `-k`: Convert a word to katakana and add it to the dictionary. (Don't use auto detection);
///     * `-K`: Add kanji to the kanji field. (Don't perform any kind of processing);
///     * `-m`: Add text to the meaning field. (Don't perform any kind of processing);
///
///         NOTE: At least one of `-r`, `-h` and `-k` must be used, but if more than one of these is present
///         they will be checked in order and only the first match will taken in account.
///         Order of precedence is `-r`, `-h` and `-k`.
///         `-m` is required (afterall, this is supposed to be a "dictionary").
///
/// * `search`: Search for a word and (if it exists in the dictionary) give back its entry.
///     * `-r`: Autodetect and convert words according to case;
///     * `-h`: Don't autodetect, convert everything to hiragana;
///     * `-k`: Don't autodetect, convert everything to katakana;
///     * `-K`: Add kanji to the entry of the input word (i.e., don't convert);
///     * `-m`: Add input as the meaning;
///
///         NOTE: At least one of these must be used. IF more than one is used (TBD):
///             1. Use a flag to determine what to do;
///             2. Show entries that match all of the options used;
///             3. Show entries that match at least one of the options used;
///
/// * `convert`: Convert everything to kana. (NOTE: maybe have this as an external tool/crate?)
///     * `-r`: Autodetect and convert words according to case;
///     * `-h`: Don't autodetect, convert everything to hiragana;
///     * `-k`: Don't autodetect, convert everything to katakana;
///
///         NOTE: At least one of these must be used. If more than one is used (TBD):
///             * Process every option;
///             * Check options in order and process only the first one;
///
fn clap() -> ArgMatches<'static> {
    // default settings for the common args between subcommands
    // needs more testing cuz i suck at rust
    // think App has Clone
    let romaji = Arg::with_name("romaji")
        .long("romaji")
        .short("r")
        .takes_value(true)
        .multiple(true);
    let hiragana = Arg::with_name("hiragana")
        .long("hiragana")
        .short("h")
        .takes_value(true)
        .multiple(true);
    let katakana = Arg::with_name("katakana")
        .long("katakana")
        .short("k")
        .takes_value(true)
        .multiple(true);
    let meaning = Arg::with_name("meaning")
        .long("meaning")
        .short("m")
        .takes_value(true)
        .multiple(true);
    let kanji = Arg::with_name("kanji")
        .long("kanji")
        .short("K")
        .takes_value(true);

    App::new("Japanese Command-line Dictionary")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Dictionary to keep track of learned/seen words and Romaji to Kana converter")
        .help_short("H")
        .subcommand(SubCommand::with_name("add")
            .about("Add an entry to the dictionary.")
            .args(&[romaji.clone()
                        .help("Convert a word and add to the dictionary."),
                    hiragana.clone()
                        .help("Convert a word to hiragana and add it to the dictionary."),
                    katakana.clone()
                        .help("Convert a word to katakana and add it to the dictionary."),
                    kanji.clone()
                        .help("Add kanji to the kanji field."),
                    meaning.clone()
                        .help("Add text to the meaning field.")
                        .required(true)]))
        .subcommand(SubCommand::with_name("search")
            .about("Search the dictionary.")
            .args(&[romaji.clone()
                        .help("Convert a word to kana and search in the dictionary."),
                    hiragana.clone()
                        .help("Convert a word to hiragana and search in the dictionary."),
                    katakana.clone()
                        .help("Convert a word to katakana and search in the dictionary."),
                    kanji.clone()
                        .help("Searches for kanji in the kanji field."),
                    meaning.clone()
                        .help("Searches for word(s) in the meaning field.")]))
        .subcommand(SubCommand::with_name("convert")
            .about("Convert text to kana.")
            .args(&[romaji.clone()
                        .help("Convert a word to kana and search in the dictionary."),
                    hiragana.clone()
                        .help("Convert a word to hiragana and search in the dictionary."),
                    katakana.clone()
                        .help("Convert a word to katakana and search in the dictionary.")]))
        .get_matches()
}
