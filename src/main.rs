use std::io::BufWriter;
use std::io::prelude::*;

#[macro_use]
extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand, Values};

extern crate r2k;
use r2k::conv_type::ConvType;
use r2k::kana_table::KanaTable;

mod io;

fn main() {
    fn handle_convert(k: &KanaTable, m: &ArgMatches) {
        fn aux<T>(k: &KanaTable, ct: ConvType<T>, ovals: Option<Values>) {
            if let Some(vals) = ovals {
                let vals: String = vals.into_iter().collect();
                println!("{}", k.convert(ct.map(|_| &vals)));
            }
        }

        aux(k, ConvType::Auto(()), m.values_of("romaji"));
        aux(k, ConvType::Hira(()), m.values_of("hiragana"));
        aux(k, ConvType::Kata(()), m.values_of("katakana"));
    }

    fn handle_add(_d: &KanaTable, _m: &ArgMatches) {
        // TODO: Implement this.
        let d_file = match io::get_file() {
            None => panic!("Error opening/creating the dictionary file."),
            Some(f) => f,
        };

        let mut filein = BufWriter::new(&d_file);
        if let Err(e) = filein.write("test test".as_bytes()) {
            panic!("oh shiet: {}", e);
        }

        unimplemented!();
    }

    fn handle_search(_d: &KanaTable, _m: &ArgMatches) {
        // TODO: Implement this.
        unimplemented!();
    }

    let map: KanaTable = KanaTable::new();

    // Get the subcommand invoked and associated arguments.
    let matches: ArgMatches = clap();
    let (cmd, matches) = matches.subcommand();
    let matches = match matches {
        Some(s) => s,
        None => return,
    };

    match cmd {
        "add" => handle_add(&map, matches),
        "convert" => handle_convert(&map, matches),
        "search" => handle_search(&map, matches),
        _ => unreachable!(),
    }
}

///
/// Usage: (This comment will be used to describe the expected behavior and the program must fit
/// this description, not the other way around)
///
/// - [ ] `add`: Add a word to the dictionary.
///     - [ ] `-r`: Convert a word and add it to the dictionary. (Use auto detection);
///     - [ ] `-h`: Convert a word to hiragana and add it to the dictionary. (Don't use auto detection);
///     - [ ] `-k`: Convert a word to katakana and add it to the dictionary. (Don't use auto detection);
///     - [ ] `-K`: Add kanji to the kanji field. (Don't perform any kind of processing);
///     - [ ] `-m`: Add text to the meaning field. (Don't perform any kind of processing);
///
///         NOTE: At least one of `-r`, `-h` and `-k` must be used, but if more than one of these is present
///         they will be checked in order and only the first match will taken in account.
///         Order of precedence is `-r`, `-h` and `-k`.
///         `-m` is required (afterall, this is supposed to be a "dictionary").
///
/// - [ ] `search`: Search for a word and (if it exists in the dictionary) give back its entry.
///     - [ ] `-r`: Autodetect and convert words according to case;
///     - [ ] `-h`: Don't autodetect, convert everything to hiragana;
///     - [ ] `-k`: Don't autodetect, convert everything to katakana;
///     - [ ] `-K`: Add kanji to the entry of the input word (i.e., don't convert);
///     - [ ] `-m`: Add input as the meaning;
///
///         NOTE: At least one of these must be used. IF more than one is used (TBD):
///             1. Use a flag to determine what to do;
///             2. Show entries that match all of the options used;
///             3. Show entries that match at least one of the options used;
///
/// - [X] `convert`: Convert everything to kana. (NOTE: maybe have this as an external tool/crate?)
///     - [X] `-r`: Autodetect and convert words according to case;
///     - [X] `-h`: Don't autodetect, convert everything to hiragana;
///     - [X] `-k`: Don't autodetect, convert everything to katakana;
///
///         NOTE: At least one of these must be used. If more than one is used:
///             - [X] **Process every option;** (Current behavior, makes more sense out of the two)
///             - [ ] ~~Check options in order and process only the first one;~~
///
fn clap() -> ArgMatches<'static> {
    // default settings for the common args between subcommands
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
        .takes_value(true)
        .multiple(true); // If there are spaces between chars
    // they're counted as multiple values
    // and the program crashes

    App::new("Japanese Command-line Dictionary")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Dictionary to keep track of learned/seen words and Romaji to Kana converter")
        .help_short("H")
        .subcommand(SubCommand::with_name("add")
                    .about("Add an entry to the dictionary.")
                    .args(&[romaji.clone()
                          .help("Convert a word and add to the dictionary.")
                          .required_unless_one(&["hiragana", "katakana"]),
                          hiragana.clone()
                          .help("Convert a word to hiragana and add it to the dictionary.")
                          .required_unless_one(&["romaji", "katakana"]),
                          katakana.clone()
                          .help("Convert a word to katakana and add it to the dictionary.")
                          .required_unless_one(&["hiragana", "romaji"]),
                          kanji.clone()
                          .help("Add kanji to the kanji field."),
                          meaning.clone()
                          .help("Add text to the meaning field.")
                          .required(true)]))
        .subcommand(SubCommand::with_name("search")
                    .about("Search the dictionary.")
                    .args(&[romaji.clone()
                          .help("Convert a word to kana and search in the dictionary.")
                          .required_unless_one(&["hiragana", "katakana"]),
                          hiragana.clone()
                          .help("Convert a word to hiragana and search in the dictionary.")
                          .required_unless_one(&["katakana", "romaji"]),
                          katakana.clone()
                          .help("Convert a word to katakana and search in the dictionary.")
                          .required_unless_one(&["hiragana", "romaji"]),
                          kanji.clone()
                          .help("Searches for kanji in the kanji field."),
                          meaning.clone()
                          .help("Searches for word(s) in the meaning field.")]))
        .subcommand(SubCommand::with_name("convert")
                    .about("Convert text to kana.")
                    .args(&[romaji.clone()
                          .help("Convert text to kana."),
                          hiragana.clone()
                          .help("Convert text to hiragana."),
                          katakana.clone()
                          .help("Convert text to katakana.")]))
        .get_matches()
}
