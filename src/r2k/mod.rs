use std::cmp;

pub mod dict;

use dict::{Dict, KanaConvertionTable};

// do_work :: Dict -> String -> String
// do_work = concat $ map convert_syllable $ syllable
//
// syllable :: Dict -> String -> [String]
// syllable _ [] = []
// syllable d s@(x:xs)
//     | fst cond = (fst $ snd $ cond):(syllable d (snd $ snd $ cond))
//     | otherwise = [x]:(syllable d xs)
//     where
//          max = let len = length s in if (>) len 3 then 3 else len
//       -- aux :: Int -> Dict -> String -> (Bool, (String, String))
//          aux 0 _ _ = (false, ([], []))
//          aux _ _ [] = (false, ([], []))
//          aux n d s
//              | contains_key d (take n s) = (true, (take n s, drop n s)
//              | otherwise = syllable (n-1) d s
//          cond = aux max d s

fn syllable(d: &dict::Dict, original: &String) -> Vec<String> {
    if original.len() == 0 {
        return vec![];
    }
    let mut ret: Vec<String> = vec![];
    let maxlen = cmp::min(original.chars().count() + 1, 4);

    for l in (1..maxlen).rev() {
        let tmp: String = original.chars().take(l).collect();

        let cond = d.contains_key(&(normalize(&tmp)));
        if cond {
            ret.push(tmp);
            let skipped: String = original.chars().skip(l).collect();
            let mut rest = syllable(d, &skipped);

            ret.append(&mut rest);

            return ret;
        }
    }

    // if a match wasn't found, add the first char to ret
    let mut taken: Vec<String> = vec![original.chars().take(1).collect()];
    ret.append(&mut taken);

    let skipped: String = original.chars().skip(1).collect();
    let mut rest = syllable(d, &skipped);
    ret.append(&mut rest);

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
    let mut ret = String::new();
    let sylvec: Vec<String> = syllable(d, w)
        .iter()
        .map(|ref s| convert_syllable(d, &s))
        .collect();

    for s in sylvec {
        ret.push_str(s.as_str());
    }

    ret
}

pub fn get_dict() -> dict::Dict {
    let mut ret = Dict::new();
    ret.init();
    ret
}
