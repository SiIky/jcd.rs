mod dict;

fn get_word(d: &dict::Dict, o: &String) -> Option<String> {
    for l in (1..4).rev() {
        let ret = &o[0..l];
        if let Some(_) = d.get(&ret.to_string()) {
            return Some(ret.to_string());
        }
    }

    None
}

fn convert_word(d: &dict::Dict, ow: &String) -> String {
    let mut nw = ow.clone();

    if nw.chars().fold(false, |acc, c| acc || c.is_uppercase()) {
        nw = nw.to_uppercase();
    }

    match (*d).get(&nw) {
        Some(c) => c.clone(),
        None => ow.clone(),
    }
}

pub fn do_work(d: &dict::Dict, w: &String) -> String {
    convert_word(d, w)
}

pub fn get_dict() -> dict::Dict {
    let mut ret = dict::new();
    dict::init(&mut ret);
    ret
}
