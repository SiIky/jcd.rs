use std::collections::HashMap;

const DEFAULT_CAPACITY: usize = 213;

pub struct Dict {
    map: HashMap<String, String>,
}

impl Dict {
    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn insert(&mut self, k: &str, v: &str) {
        self.map.insert(k.to_string(), v.to_string());
    }

    pub fn get(&self, k: &String) -> Option<&String> {
        self.map.get(k)
    }

    pub fn contains_key(&self, k: &String) -> bool {
        self.map.contains_key(k)
    }

    pub fn init(&mut self) {
        if !self.is_empty() {
            return;
        }

        // Hiragana
        self.insert("a", "\u{3042}"); // あ
        self.insert("i", "\u{3044}"); // い
        self.insert("u", "\u{3046}"); // う
        self.insert("e", "\u{3048}"); // え
        self.insert("o", "\u{304a}"); // お
        self.insert("ka", "\u{304b}"); // か
        self.insert("ga", "\u{304c}"); // が
        self.insert("ki", "\u{304d}"); // き
        self.insert("gi", "\u{304e}"); // ぎ
        self.insert("ku", "\u{304f}"); // く
        self.insert("gu", "\u{3050}"); // ぐ
        self.insert("ke", "\u{3051}"); // け
        self.insert("ge", "\u{3052}"); // げ
        self.insert("ko", "\u{3053}"); // こ
        self.insert("go", "\u{3054}"); // ご
        self.insert("sa", "\u{3055}"); // さ
        self.insert("za", "\u{3056}"); // ざ
        self.insert("shi", "\u{3057}"); // し
        self.insert("ji", "\u{3058}"); // じ
        self.insert("su", "\u{3059}"); // す
        self.insert("zu", "\u{305a}"); // ず
        self.insert("se", "\u{305b}"); // せ
        self.insert("ze", "\u{305c}"); // ぜ
        self.insert("so", "\u{305d}"); // そ
        self.insert("zo", "\u{305e}"); // ぞ
        self.insert("ta", "\u{305f}"); // た
        self.insert("da", "\u{3060}"); // だ
        self.insert("chi", "\u{3061}"); // ち
        self.insert("dji", "\u{3062}"); // ぢ
        self.insert("tsu", "\u{3064}"); // つ
        self.insert("dzu", "\u{3065}"); // づ
        self.insert("te", "\u{3066}"); // て
        self.insert("de", "\u{3067}"); // で
        self.insert("to", "\u{3068}"); // と
        self.insert("do", "\u{3069}"); // ど
        self.insert("na", "\u{306a}"); // な
        self.insert("ni", "\u{306b}"); // に
        self.insert("nu", "\u{306c}"); // ぬ
        self.insert("ne", "\u{306d}"); // ね
        self.insert("no", "\u{306d}"); // の
        self.insert("ha", "\u{306f}"); // は
        self.insert("ba", "\u{3070}"); // ば
        self.insert("pa", "\u{3071}"); // ぱ
        self.insert("hi", "\u{3072}"); // ひ
        self.insert("bi", "\u{3073}"); // び
        self.insert("pi", "\u{3074}"); // ぴ
        self.insert("fu", "\u{3075}"); // ふ
        self.insert("bu", "\u{3076}"); // ぶ
        self.insert("pu", "\u{3077}"); // ぷ
        self.insert("he", "\u{3078}"); // へ
        self.insert("be", "\u{3079}"); // べ
        self.insert("pe", "\u{307a}"); // ぺ
        self.insert("ho", "\u{307b}"); // ほ
        self.insert("bo", "\u{307c}"); // ぼ
        self.insert("po", "\u{307d}"); // ぽ
        self.insert("ma", "\u{307e}"); // ま
        self.insert("mi", "\u{307f}"); // み
        self.insert("mu", "\u{3080}"); // む
        self.insert("me", "\u{3081}"); // め
        self.insert("mo", "\u{3082}"); // も
        self.insert("ya", "\u{3084}"); // や
        self.insert("yu", "\u{3086}"); // ゆ
        self.insert("yo", "\u{3088}"); // よ
        self.insert("ra", "\u{3089}"); // ら
        self.insert("ri", "\u{308a}"); // り
        self.insert("ru", "\u{308b}"); // る
        self.insert("re", "\u{308c}"); // れ
        self.insert("ro", "\u{308d}"); // ろ
        self.insert("wa", "\u{308f}"); // わ
        self.insert("kya", "\u{304d}\u{3083}"); // き
        self.insert("kyu", "\u{304d}\u{3085}"); // き
        self.insert("kyo", "\u{304d}\u{3087}"); // き
        self.insert("gya", "\u{304e}\u{3083}"); //
        self.insert("gyu", "\u{304e}\u{3085}"); //
        self.insert("gyo", "\u{304e}\u{3087}"); //
        self.insert("sha", "\u{3057}\u{3083}"); //
        self.insert("shu", "\u{3057}\u{3085}"); //
        self.insert("sho", "\u{3057}\u{3087}"); //
        self.insert("ja", "\u{3058}\u{3083}"); //
        self.insert("ju", "\u{3058}\u{3085}"); //
        self.insert("jo", "\u{3058}\u{3087}"); //
        self.insert("cha", "\u{3061}\u{3083}"); //
        self.insert("chu", "\u{3061}\u{3085}"); //
        self.insert("cho", "\u{3061}\u{3087}"); //
        self.insert("dja", "\u{3062}\u{3083}"); //
        self.insert("dju", "\u{3062}\u{3085}"); //
        self.insert("djo", "\u{3062}\u{3087}"); //
        self.insert("hya", "\u{3072}\u{3083}"); //
        self.insert("hyu", "\u{3072}\u{3085}"); //
        self.insert("hyo", "\u{3072}\u{3087}"); //
        self.insert("bya", "\u{3073}\u{3083}"); //
        self.insert("byu", "\u{3073}\u{3085}"); //
        self.insert("byo", "\u{3073}\u{3087}"); //
        self.insert("pya", "\u{3074}\u{3083}"); //
        self.insert("pyu", "\u{3074}\u{3085}"); //
        self.insert("pyo", "\u{3074}\u{3087}"); //
        self.insert("nya", "\u{306b}\u{3083}"); //
        self.insert("nyu", "\u{306b}\u{3085}"); //
        self.insert("nyo", "\u{306b}\u{3087}"); //
        self.insert("wi", "\u{3090}"); // ゐ
        self.insert("we", "\u{3091}"); // ゑ
        self.insert("wo", "\u{3092}"); // を
        self.insert("n", "\u{3093}"); // ん
        self.insert("vu", "\u{3094}"); // ゔ

        // Katakana
        self.insert("A", "\u{30a2}"); // ア
        self.insert("I", "\u{30a4}"); // イ
        self.insert("U", "\u{30a6}"); // ウ
        self.insert("E", "\u{30a8}"); // エ
        self.insert("O", "\u{30aa}"); // オ
        self.insert("KA", "\u{30ab}"); // カ
        self.insert("GA", "\u{30ac}"); // ガ
        self.insert("KI", "\u{30ad}"); // キ
        self.insert("GI", "\u{30ae}"); // ギ
        self.insert("KU", "\u{30af}"); // ク
        self.insert("GU", "\u{30b0}"); // グ
        self.insert("KE", "\u{30b1}"); // ケ
        self.insert("GE", "\u{30b2}"); // ゲ
        self.insert("KO", "\u{30b3}"); // コ
        self.insert("GO", "\u{30b4}"); // ゴ
        self.insert("SA", "\u{30b5}"); // サ
        self.insert("ZA", "\u{30b6}"); // ザ
        self.insert("SHI", "\u{30b7}"); // シ
        self.insert("JI", "\u{30b8}"); // ジ
        self.insert("SU", "\u{30b9}"); // ス
        self.insert("ZU", "\u{30ba}"); // ズ
        self.insert("SE", "\u{30bb}"); // セ
        self.insert("ZE", "\u{30bc}"); // ゼ
        self.insert("SO", "\u{30bd}"); // ソ
        self.insert("ZO", "\u{30be}"); // ゾ
        self.insert("TA", "\u{30bf}"); // タ
        self.insert("DA", "\u{30c0}"); // ダ
        self.insert("CHI", "\u{30c1}"); // チ
        self.insert("DJI", "\u{30c2}"); // ヂ
        self.insert("TSU", "\u{30c4}"); // ツ
        self.insert("DZU", "\u{30c5}"); // ヅ
        self.insert("TE", "\u{30c6}"); // テ
        self.insert("DE", "\u{30c7}"); // デ
        self.insert("TO", "\u{30c8}"); // ト
        self.insert("DO", "\u{30c9}"); // ド
        self.insert("NA", "\u{30ca}"); // ナ
        self.insert("NI", "\u{30cb}"); // ニ
        self.insert("NU", "\u{30cc}"); // ヌ
        self.insert("NE", "\u{30cd}"); // ネ
        self.insert("NO", "\u{30ce}"); // ノ
        self.insert("HA", "\u{30cf}"); // ハ
        self.insert("BA", "\u{30d0}"); // バ
        self.insert("PA", "\u{30d1}"); // パ
        self.insert("HI", "\u{30d2}"); // ヒ
        self.insert("BI", "\u{30d3}"); // ビ
        self.insert("PI", "\u{30d4}"); // ピ
        self.insert("FU", "\u{30d5}"); // フ
        self.insert("BU", "\u{30d6}"); // ブ
        self.insert("PU", "\u{30d7}"); // プ
        self.insert("HE", "\u{30d8}"); // ヘ
        self.insert("BE", "\u{30d9}"); // ベ
        self.insert("PE", "\u{30da}"); // ペ
        self.insert("HO", "\u{30db}"); // ホ
        self.insert("BO", "\u{30dc}"); // ボ
        self.insert("PO", "\u{30dd}"); // ポ
        self.insert("MA", "\u{30de}"); // マ
        self.insert("MI", "\u{30df}"); // ミ
        self.insert("MU", "\u{30e0}"); // ム
        self.insert("ME", "\u{30e1}"); // メ
        self.insert("MO", "\u{30e2}"); // モ
        self.insert("YA", "\u{30e4}"); // ヤ
        self.insert("YU", "\u{30e6}"); // ユ
        self.insert("YO", "\u{30e8}"); // ヨ
        self.insert("RA", "\u{30e9}"); // ラ
        self.insert("RI", "\u{30ea}"); // リ
        self.insert("RU", "\u{30eb}"); // ル
        self.insert("RE", "\u{30ec}"); // レ
        self.insert("RO", "\u{30ed}"); // ロ
        self.insert("WA", "\u{30ef}"); // ワ
        self.insert("KYA", "\u{30ad}\u{30e3}"); //
        self.insert("KYU", "\u{30ad}\u{30e5}"); //
        self.insert("KYO", "\u{30ad}\u{30e7}"); //
        self.insert("GYA", "\u{30ae}\u{30e3}"); //
        self.insert("GYU", "\u{30ae}\u{30e5}"); //
        self.insert("GYO", "\u{30ae}\u{30e7}"); //
        self.insert("SHA", "\u{30b7}\u{30e3}"); //
        self.insert("SHU", "\u{30b7}\u{30e5}"); //
        self.insert("SHO", "\u{30b7}\u{30e7}"); //
        self.insert("JA", "\u{30b8}\u{30e3}"); //
        self.insert("JU", "\u{30b8}\u{30e5}"); //
        self.insert("JO", "\u{30b8}\u{30e7}"); //
        self.insert("CHA", "\u{30c1}\u{30e3}"); //
        self.insert("CHU", "\u{30c1}\u{30e5}"); //
        self.insert("CHO", "\u{30c1}\u{30e7}"); //
        self.insert("DJA", "\u{30c2}\u{30e3}"); //
        self.insert("DJU", "\u{30c2}\u{30e5}"); //
        self.insert("DJO", "\u{30c2}\u{30e7}"); //
        self.insert("HYA", "\u{30d2}\u{30e3}"); //
        self.insert("HYU", "\u{30d2}\u{30e5}"); //
        self.insert("HYO", "\u{30d2}\u{30e7}"); //
        self.insert("BYA", "\u{30d3}\u{30e3}"); //
        self.insert("BYU", "\u{30d3}\u{30e5}"); //
        self.insert("BYO", "\u{30d3}\u{30e7}"); //
        self.insert("PYA", "\u{30d4}\u{30e3}"); //
        self.insert("PYU", "\u{30d4}\u{30e5}"); //
        self.insert("PYO", "\u{30d4}\u{30e7}"); //
        self.insert("NYA", "\u{30cb}\u{30e3}"); //
        self.insert("NYU", "\u{30cb}\u{30e5}"); //
        self.insert("NYO", "\u{30cb}\u{30e7}"); //
        self.insert("WI", "\u{30f0}"); // ヰ
        self.insert("WE", "\u{30f1}"); // ヱ
        self.insert("WO", "\u{30f2}"); // ヲ
        self.insert("N", "\u{30f3}"); // ン
        self.insert("VU", "\u{30f4}"); // ヴ

        // Punctuation
        self.insert(".", "\u{3002}"); // 。
        self.insert(",", "\u{3001}"); // 、
        self.insert("!", "\u{ff01}"); // ！
        self.insert("?", "\u{ff1f}"); // ？
        self.insert("-", "\u{30fc}"); // ー
    }

    pub fn new() -> Dict {
        Dict { map: HashMap::with_capacity(DEFAULT_CAPACITY) }
    }
}
