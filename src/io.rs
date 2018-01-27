use std::env;
use std::fs::File;
use std::path::PathBuf;

/// Dictionary file encoded in JSON with the following structure
///
/// ```json
/// [                       // The dictionary is an array of entries
///     {                   // Each dictionary entry is an object
///         "kana": ""      // The word without kanji
///         "kanji": ""     // The word including kanji (if any)
///         "meaning": ""   // The meaning of the word
///     },
/// ]
/// ```
///

fn path() -> Option<PathBuf> {
    let ret = match (env::var("JCD_DIR"), env::var("HOME")) {
        (Ok(j), _) => j,
        (_, Ok(h)) => h,
        _ => return None,
    };

    let mut ret = PathBuf::from(ret);
    ret.push("dict.txt");

    Some(ret)
}

pub fn get_file() -> Option<File> {
    match path() {
        Some(p) => match File::open(&p) {
            Err(_) => File::create(&p).ok(),
            f => f.ok(),
        },
        None => None,
    }
}

//pub fn search_file(f: &File, s: String) -> Option<String> {
//    unimplemented!();
//}

//pub fn add_file(f: &File, s: String) -> Option<File> {
//    unimplemented!();
//}
