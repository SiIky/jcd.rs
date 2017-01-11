use std::collections::HashMap;

const DEFAULT_CAPACITY: usize = 213;

// type Dict = HashMap<String, String>;
pub struct Dict {
    map: HashMap<String, String>,
}

// for later
// trait ConvertionTable { fn new() -> Dict; fn init(&mut self); }
// impl ConvertionTable for HashMap<String, String> {
impl Dict {
    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn insert(&mut self, k: String, v: String) {
        self.map.insert(k, v);
    }

    pub fn get(&self, k: &String) -> Option<&String> {
        self.map.get(k)
    }

    pub fn contains_key(&self, k: &String) -> bool {
        self.map.contains_key(k)
    }

    fn clear(&mut self) {
        self.map.clear();
    }

    pub fn init(&mut self) {
        if !self.is_empty() {
            self.clear();
        }

        let pairs: Vec<(String, String)> = (vec![// Hiragana
                                                 ("a", "\u{3042}"), // あ
                                                 ("i", "\u{3044}"), // い
                                                 ("u", "\u{3046}"), // う
                                                 ("e", "\u{3048}"), // え
                                                 ("o", "\u{304a}"), // お
                                                 ("ka", "\u{304b}"), // か
                                                 ("ga", "\u{304c}"), // が
                                                 ("ki", "\u{304d}"), // き
                                                 ("gi", "\u{304e}"), // ぎ
                                                 ("ku", "\u{304f}"), // く
                                                 ("gu", "\u{3050}"), // ぐ
                                                 ("ke", "\u{3051}"), // け
                                                 ("ge", "\u{3052}"), // げ
                                                 ("ko", "\u{3053}"), // こ
                                                 ("go", "\u{3054}"), // ご
                                                 ("sa", "\u{3055}"), // さ
                                                 ("za", "\u{3056}"), // ざ
                                                 ("shi", "\u{3057}"), // し
                                                 ("ji", "\u{3058}"), // じ
                                                 ("su", "\u{3059}"), // す
                                                 ("zu", "\u{305a}"), // ず
                                                 ("se", "\u{305b}"), // せ
                                                 ("ze", "\u{305c}"), // ぜ
                                                 ("so", "\u{305d}"), // そ
                                                 ("zo", "\u{305e}"), // ぞ
                                                 ("ta", "\u{305f}"), // た
                                                 ("da", "\u{3060}"), // だ
                                                 ("chi", "\u{3061}"), // ち
                                                 ("dji", "\u{3062}"), // ぢ
                                                 ("tsu", "\u{3064}"), // つ
                                                 ("dzu", "\u{3065}"), // づ
                                                 ("te", "\u{3066}"), // て
                                                 ("de", "\u{3067}"), // で
                                                 ("to", "\u{3068}"), // と
                                                 ("do", "\u{3069}"), // ど
                                                 ("na", "\u{306a}"), // な
                                                 ("ni", "\u{306b}"), // に
                                                 ("nu", "\u{306c}"), // ぬ
                                                 ("ne", "\u{306d}"), // ね
                                                 ("no", "\u{306d}"), // の
                                                 ("ha", "\u{306f}"), // は
                                                 ("ba", "\u{3070}"), // ば
                                                 ("pa", "\u{3071}"), // ぱ
                                                 ("hi", "\u{3072}"), // ひ
                                                 ("bi", "\u{3073}"), // び
                                                 ("pi", "\u{3074}"), // ぴ
                                                 ("fu", "\u{3075}"), // ふ
                                                 ("bu", "\u{3076}"), // ぶ
                                                 ("pu", "\u{3077}"), // ぷ
                                                 ("he", "\u{3078}"), // へ
                                                 ("be", "\u{3079}"), // べ
                                                 ("pe", "\u{307a}"), // ぺ
                                                 ("ho", "\u{307b}"), // ほ
                                                 ("bo", "\u{307c}"), // ぼ
                                                 ("po", "\u{307d}"), // ぽ
                                                 ("ma", "\u{307e}"), // ま
                                                 ("mi", "\u{307f}"), // み
                                                 ("mu", "\u{3080}"), // む
                                                 ("me", "\u{3081}"), // め
                                                 ("mo", "\u{3082}"), // も
                                                 ("ya", "\u{3084}"), // や
                                                 ("yu", "\u{3086}"), // ゆ
                                                 ("yo", "\u{3088}"), // よ
                                                 ("ra", "\u{3089}"), // ら
                                                 ("ri", "\u{308a}"), // り
                                                 ("ru", "\u{308b}"), // る
                                                 ("re", "\u{308c}"), // れ
                                                 ("ro", "\u{308d}"), // ろ
                                                 ("wa", "\u{308f}"), // わ
                                                 ("kya", "\u{304d}\u{3083}"), // き
                                                 ("kyu", "\u{304d}\u{3085}"), // き
                                                 ("kyo", "\u{304d}\u{3087}"), // き
                                                 ("gya", "\u{304e}\u{3083}"), //
                                                 ("gyu", "\u{304e}\u{3085}"), //
                                                 ("gyo", "\u{304e}\u{3087}"), //
                                                 ("sha", "\u{3057}\u{3083}"), //
                                                 ("shu", "\u{3057}\u{3085}"), //
                                                 ("sho", "\u{3057}\u{3087}"), //
                                                 ("ja", "\u{3058}\u{3083}"), //
                                                 ("ju", "\u{3058}\u{3085}"), //
                                                 ("jo", "\u{3058}\u{3087}"), //
                                                 ("cha", "\u{3061}\u{3083}"), //
                                                 ("chu", "\u{3061}\u{3085}"), //
                                                 ("cho", "\u{3061}\u{3087}"), //
                                                 ("dja", "\u{3062}\u{3083}"), //
                                                 ("dju", "\u{3062}\u{3085}"), //
                                                 ("djo", "\u{3062}\u{3087}"), //
                                                 ("hya", "\u{3072}\u{3083}"), //
                                                 ("hyu", "\u{3072}\u{3085}"), //
                                                 ("hyo", "\u{3072}\u{3087}"), //
                                                 ("bya", "\u{3073}\u{3083}"), //
                                                 ("byu", "\u{3073}\u{3085}"), //
                                                 ("byo", "\u{3073}\u{3087}"), //
                                                 ("pya", "\u{3074}\u{3083}"), //
                                                 ("pyu", "\u{3074}\u{3085}"), //
                                                 ("pyo", "\u{3074}\u{3087}"), //
                                                 ("nya", "\u{306b}\u{3083}"), //
                                                 ("nyu", "\u{306b}\u{3085}"), //
                                                 ("nyo", "\u{306b}\u{3087}"), //
                                                 ("wi", "\u{3090}"), // ゐ
                                                 ("we", "\u{3091}"), // ゑ
                                                 ("wo", "\u{3092}"), // を
                                                 ("n", "\u{3093}"), // ん
                                                 ("vu", "\u{3094}"), // ゔ
                                                 // lazy ass aproach to pauses (small tsu) goes here
                                                 //
                                                 // Katakana
                                                 ("A", "\u{30a2}"), // ア
                                                 ("I", "\u{30a4}"), // イ
                                                 ("U", "\u{30a6}"), // ウ
                                                 ("E", "\u{30a8}"), // エ
                                                 ("O", "\u{30aa}"), // オ
                                                 ("KA", "\u{30ab}"), // カ
                                                 ("GA", "\u{30ac}"), // ガ
                                                 ("KI", "\u{30ad}"), // キ
                                                 ("GI", "\u{30ae}"), // ギ
                                                 ("KU", "\u{30af}"), // ク
                                                 ("GU", "\u{30b0}"), // グ
                                                 ("KE", "\u{30b1}"), // ケ
                                                 ("GE", "\u{30b2}"), // ゲ
                                                 ("KO", "\u{30b3}"), // コ
                                                 ("GO", "\u{30b4}"), // ゴ
                                                 ("SA", "\u{30b5}"), // サ
                                                 ("ZA", "\u{30b6}"), // ザ
                                                 ("SHI", "\u{30b7}"), // シ
                                                 ("JI", "\u{30b8}"), // ジ
                                                 ("SU", "\u{30b9}"), // ス
                                                 ("ZU", "\u{30ba}"), // ズ
                                                 ("SE", "\u{30bb}"), // セ
                                                 ("ZE", "\u{30bc}"), // ゼ
                                                 ("SO", "\u{30bd}"), // ソ
                                                 ("ZO", "\u{30be}"), // ゾ
                                                 ("TA", "\u{30bf}"), // タ
                                                 ("DA", "\u{30c0}"), // ダ
                                                 ("CHI", "\u{30c1}"), // チ
                                                 ("DJI", "\u{30c2}"), // ヂ
                                                 ("TSU", "\u{30c4}"), // ツ
                                                 ("DZU", "\u{30c5}"), // ヅ
                                                 ("TE", "\u{30c6}"), // テ
                                                 ("DE", "\u{30c7}"), // デ
                                                 ("TO", "\u{30c8}"), // ト
                                                 ("DO", "\u{30c9}"), // ド
                                                 ("NA", "\u{30ca}"), // ナ
                                                 ("NI", "\u{30cb}"), // ニ
                                                 ("NU", "\u{30cc}"), // ヌ
                                                 ("NE", "\u{30cd}"), // ネ
                                                 ("NO", "\u{30ce}"), // ノ
                                                 ("HA", "\u{30cf}"), // ハ
                                                 ("BA", "\u{30d0}"), // バ
                                                 ("PA", "\u{30d1}"), // パ
                                                 ("HI", "\u{30d2}"), // ヒ
                                                 ("BI", "\u{30d3}"), // ビ
                                                 ("PI", "\u{30d4}"), // ピ
                                                 ("FU", "\u{30d5}"), // フ
                                                 ("BU", "\u{30d6}"), // ブ
                                                 ("PU", "\u{30d7}"), // プ
                                                 ("HE", "\u{30d8}"), // ヘ
                                                 ("BE", "\u{30d9}"), // ベ
                                                 ("PE", "\u{30da}"), // ペ
                                                 ("HO", "\u{30db}"), // ホ
                                                 ("BO", "\u{30dc}"), // ボ
                                                 ("PO", "\u{30dd}"), // ポ
                                                 ("MA", "\u{30de}"), // マ
                                                 ("MI", "\u{30df}"), // ミ
                                                 ("MU", "\u{30e0}"), // ム
                                                 ("ME", "\u{30e1}"), // メ
                                                 ("MO", "\u{30e2}"), // モ
                                                 ("YA", "\u{30e4}"), // ヤ
                                                 ("YU", "\u{30e6}"), // ユ
                                                 ("YO", "\u{30e8}"), // ヨ
                                                 ("RA", "\u{30e9}"), // ラ
                                                 ("RI", "\u{30ea}"), // リ
                                                 ("RU", "\u{30eb}"), // ル
                                                 ("RE", "\u{30ec}"), // レ
                                                 ("RO", "\u{30ed}"), // ロ
                                                 ("WA", "\u{30ef}"), // ワ
                                                 ("KYA", "\u{30ad}\u{30e3}"), //
                                                 ("KYU", "\u{30ad}\u{30e5}"), //
                                                 ("KYO", "\u{30ad}\u{30e7}"), //
                                                 ("GYA", "\u{30ae}\u{30e3}"), //
                                                 ("GYU", "\u{30ae}\u{30e5}"), //
                                                 ("GYO", "\u{30ae}\u{30e7}"), //
                                                 ("SHA", "\u{30b7}\u{30e3}"), //
                                                 ("SHU", "\u{30b7}\u{30e5}"), //
                                                 ("SHO", "\u{30b7}\u{30e7}"), //
                                                 ("JA", "\u{30b8}\u{30e3}"), //
                                                 ("JU", "\u{30b8}\u{30e5}"), //
                                                 ("JO", "\u{30b8}\u{30e7}"), //
                                                 ("CHA", "\u{30c1}\u{30e3}"), //
                                                 ("CHU", "\u{30c1}\u{30e5}"), //
                                                 ("CHO", "\u{30c1}\u{30e7}"), //
                                                 ("DJA", "\u{30c2}\u{30e3}"), //
                                                 ("DJU", "\u{30c2}\u{30e5}"), //
                                                 ("DJO", "\u{30c2}\u{30e7}"), //
                                                 ("HYA", "\u{30d2}\u{30e3}"), //
                                                 ("HYU", "\u{30d2}\u{30e5}"), //
                                                 ("HYO", "\u{30d2}\u{30e7}"), //
                                                 ("BYA", "\u{30d3}\u{30e3}"), //
                                                 ("BYU", "\u{30d3}\u{30e5}"), //
                                                 ("BYO", "\u{30d3}\u{30e7}"), //
                                                 ("PYA", "\u{30d4}\u{30e3}"), //
                                                 ("PYU", "\u{30d4}\u{30e5}"), //
                                                 ("PYO", "\u{30d4}\u{30e7}"), //
                                                 ("NYA", "\u{30cb}\u{30e3}"), //
                                                 ("NYU", "\u{30cb}\u{30e5}"), //
                                                 ("NYO", "\u{30cb}\u{30e7}"), //
                                                 ("WI", "\u{30f0}"), // ヰ
                                                 ("WE", "\u{30f1}"), // ヱ
                                                 ("WO", "\u{30f2}"), // ヲ
                                                 ("N", "\u{30f3}"), // ン
                                                 ("VU", "\u{30f4}"), // ヴ
                                                 // lazy ass aproach to long vowels in Katakana
                                                 ("AA", "\u{30a2}\u{30fc}"), // ア
                                                 ("II", "\u{30a4}\u{30fc}"), // イ
                                                 ("UU", "\u{30a6}\u{30fc}"), // ウ
                                                 ("EE", "\u{30a8}\u{30fc}"), // エ
                                                 ("OO", "\u{30aa}\u{30fc}"), // オ
                                                 // lazy ass aproach to pauses (small tsu) goes here
                                                 //
                                                 // Punctuation
                                                 (".", "\u{3002}"), // 。
                                                 (",", "\u{3001}"), // 、
                                                 ("!", "\u{ff01}"), // ！
                                                 ("?", "\u{ff1f}"), // ？
                                                 ("-", "\u{30fc}") /* ー */])
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        for (k, v) in pairs {
            self.insert(k, v);
        }
    }

    pub fn new() -> Dict {
        Dict { map: HashMap::with_capacity(DEFAULT_CAPACITY) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print_all() {
        let mut d = Dict::new();
        d.init();
        for k in d.map.keys() {
            println!("{}\t=>{}", k, d.get(&k).unwrap());
        }
    }
}
