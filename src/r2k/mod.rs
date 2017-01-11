use std::cmp;

mod dict;

fn syllable(d: &dict::Dict, original: &String) -> Vec<String> {
    let mut ret: Vec<String> = vec![];
    let maxlen = cmp::min(original.chars().count() + 1, 4);

    for l in (1..maxlen).rev() {
        let tmp = original.chars().take(l).collect();

        if d.contains_key(&(normalize(&tmp))) {
            let skipped = original.chars().skip(l).collect();
            let mut rest = syllable(d, &skipped);

            ret.append(&mut rest);
            break;
        }
    }

    ret
}

fn normalize(s: &String) -> String {
    match s.chars().fold(false, |acc, c| acc || c.is_uppercase()) {
        true => s.to_uppercase(),
        false => s.clone(),
    }
}

fn convert_syllable(d: &dict::Dict, ow: &String) -> String {
    match d.get(&normalize(&ow)) {
        Some(c) => c.clone(),
        None => ow.clone(),
    }
}

pub fn do_work(d: &dict::Dict, w: &String) -> String {
    let sylvec: Vec<String> = syllable(d, w)
        .iter()
        .map(|&s| convert_syllable(d, &s))
        .collect();

    let mut tmp = String::new();
    let ret: String = sylvec.iter()
        .fold(tmp, |ref mut acc, &s| acc.push_str(s.as_ref()));

    ret
}

pub fn get_dict() -> dict::Dict {
    let mut ret = dict::Dict::new();
    ret.init();
    ret
}
