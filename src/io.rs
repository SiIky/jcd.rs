use std::env;
use std::fs::File;
use std::path::PathBuf;

pub fn get_file() -> Option<File> {
    let jcd = env::var("JCD_DIR");
    let home = env::var("HOME");

    let base = match (jcd, home) {
        (Ok(j), _) => j,
        (_, Ok(h)) => h,
        _ => return None,
    };

    let mut base = PathBuf::from(base);
    base.push("dict.txt");
    let base = base;

    let ret = match File::open(&base) {
        Err(_) => File::create(&base), // If the dict.txt file doesn't exist, try to create it
        f => f,
    };

    match ret {
        Ok(f) => Some(f),
        Err(_) => None,
    }
}
