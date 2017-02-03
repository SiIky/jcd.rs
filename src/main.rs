#[macro_use]
extern crate clap;
// extern crate r2k;
mod r2k;

use clap::{App, Arg, ArgMatches, SubCommand, Values};
use r2k::dict;

mod io;

fn main() {
    let _file = match io::get_file() {
        None => return,
        Some(f) => f,
    };

    let map: dict::Dict = r2k::get_dict();

    println!("before");
    let matches = clap();

    let (subcmd, sub_matches) = matches.subcommand();
    let sub_matches = match sub_matches {
        // NOTE: maybe unreachable!() could be used instead
        None => panic!("No options given to subcommand {}", subcmd),
        Some(s) => s,
    };

    match subcmd {
        "add" => handle_add(&map, sub_matches),
        "search" => handle_search(&map, sub_matches),
        "convert" => handle_convert(&map, sub_matches),
        // NOTE: maybe unreachable!() could be used instead
        _ => panic!("Command not recognized!"),
    }

    println!("after");
}

fn handle_add(_d: &dict::Dict, _m: &ArgMatches) {
    unimplemented!();
}

fn handle_search(_d: &dict::Dict, _m: &ArgMatches) {
    unimplemented!();
}

fn handle_convert(d: &dict::Dict, m: &ArgMatches) {
    // TODO: Make a more general approach to turn `hira`/`kata`
    // to lower/upper-case. Maybe related to the TODOs below.
    let romaji = m.values_of("romaji");
    let hira = m.values_of("hiragana");
    let kata = m.values_of("katakana");

    // TODO: In case a string is not considered a key,
    // its case is changed to lowercase.
    let aux4hira = |opt: &str, valsopt: Option<Values>| {
        if let Some(vals) = valsopt {
            print!("{} args:", opt.to_string());
            let mut tmp = String::new();
            for val in vals {
                print!(" {}", val);
                let res = r2k::do_work(d, &String::from(val.to_lowercase()));
                tmp.push_str(res.as_ref());
            }
            println!("\n{} result: {}", opt, tmp);
        }
    };

    // TODO: In case a string is not considered a key,
    // its case is changed to uppercase.
    let aux4kata = |opt: &str, valsopt: Option<Values>| {
        if let Some(vals) = valsopt {
            print!("{} args:", opt.to_string());
            let mut tmp = String::new();
            for val in vals {
                print!(" {}", val);
                let res = r2k::do_work(d, &String::from(val.to_uppercase()));
                tmp.push_str(res.as_ref());
            }
            println!("\n{} result: {}", opt, tmp);
        }
    };

    let aux4romaji = |opt: &str, valsopt: Option<Values>| {
        if let Some(vals) = valsopt {
            print!("{} args:", opt.to_string());
            let mut tmp = String::new();
            for val in vals {
                print!(" {}", val);
                let res = r2k::do_work(d, &String::from(val));
                tmp.push_str(res.as_ref());
            }
            println!("\n{} result: {}", opt, tmp);
        }
    };

    aux4hira("-h", hira);
    aux4kata("-k", kata);
    aux4romaji("-r", romaji);
}

///
/// Usage: (This comment will be used to describe the expected behavior and the program must fit this
///        description, not the other way around)
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
///             - [X] **Process every option;** (Current behavior, makes the more sense out of the two)
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
