use std::collections::HashMap;

pub type Dict = HashMap<String, String>;

pub fn init(d: &mut Dict) {
    if !d.is_empty() {
        return;
    }
    // Hiragana
    d.insert("a".to_string(), "\u{3042}".to_string()); // あ
    d.insert("i".to_string(), "\u{3044}".to_string()); // い
    d.insert("u".to_string(), "\u{3046}".to_string()); // う
    d.insert("e".to_string(), "\u{3048}".to_string()); // え
    d.insert("o".to_string(), "\u{304a}".to_string()); // お
    d.insert("ka".to_string(), "\u{304b}".to_string()); // か
    d.insert("ga".to_string(), "\u{304c}".to_string()); // が
    d.insert("ki".to_string(), "\u{304d}".to_string()); // き
    d.insert("gi".to_string(), "\u{304e}".to_string()); // ぎ
    d.insert("ku".to_string(), "\u{304f}".to_string()); // く
    d.insert("gu".to_string(), "\u{3050}".to_string()); // ぐ
    d.insert("ke".to_string(), "\u{3051}".to_string()); // け
    d.insert("ge".to_string(), "\u{3052}".to_string()); // げ
    d.insert("ko".to_string(), "\u{3053}".to_string()); // こ
    d.insert("go".to_string(), "\u{3054}".to_string()); // ご
    d.insert("sa".to_string(), "\u{3055}".to_string()); // さ
    d.insert("za".to_string(), "\u{3056}".to_string()); // ざ
    d.insert("shi".to_string(), "\u{3057}".to_string()); // し
    d.insert("ji".to_string(), "\u{3058}".to_string()); // じ
    d.insert("su".to_string(), "\u{3059}".to_string()); // す
    d.insert("zu".to_string(), "\u{305a}".to_string()); // ず
    d.insert("se".to_string(), "\u{305b}".to_string()); // せ
    d.insert("ze".to_string(), "\u{305c}".to_string()); // ぜ
    d.insert("so".to_string(), "\u{305d}".to_string()); // そ
    d.insert("zo".to_string(), "\u{305e}".to_string()); // ぞ
    d.insert("ta".to_string(), "\u{305f}".to_string()); // た
    d.insert("da".to_string(), "\u{3060}".to_string()); // だ
    d.insert("chi".to_string(), "\u{3061}".to_string()); // ち
    d.insert("dji".to_string(), "\u{3062}".to_string()); // ぢ
    d.insert("tsu".to_string(), "\u{3064}".to_string()); // つ
    d.insert("dzu".to_string(), "\u{3065}".to_string()); // づ
    d.insert("te".to_string(), "\u{3066}".to_string()); // て
    d.insert("de".to_string(), "\u{3067}".to_string()); // で
    d.insert("to".to_string(), "\u{3068}".to_string()); // と
    d.insert("do".to_string(), "\u{3069}".to_string()); // ど
    d.insert("na".to_string(), "\u{306a}".to_string()); // な
    d.insert("ni".to_string(), "\u{306b}".to_string()); // に
    d.insert("nu".to_string(), "\u{306c}".to_string()); // ぬ
    d.insert("ne".to_string(), "\u{306d}".to_string()); // ね
    d.insert("no".to_string(), "\u{306d}".to_string()); // の
    d.insert("ha".to_string(), "\u{306f}".to_string()); // は
    d.insert("ba".to_string(), "\u{3070}".to_string()); // ば
    d.insert("pa".to_string(), "\u{3071}".to_string()); // ぱ
    d.insert("hi".to_string(), "\u{3072}".to_string()); // ひ
    d.insert("bi".to_string(), "\u{3073}".to_string()); // び
    d.insert("pi".to_string(), "\u{3074}".to_string()); // ぴ
    d.insert("fu".to_string(), "\u{3075}".to_string()); // ふ
    d.insert("bu".to_string(), "\u{3076}".to_string()); // ぶ
    d.insert("pu".to_string(), "\u{3077}".to_string()); // ぷ
    d.insert("he".to_string(), "\u{3078}".to_string()); // へ
    d.insert("be".to_string(), "\u{3079}".to_string()); // べ
    d.insert("pe".to_string(), "\u{307a}".to_string()); // ぺ
    d.insert("ho".to_string(), "\u{307b}".to_string()); // ほ
    d.insert("bo".to_string(), "\u{307c}".to_string()); // ぼ
    d.insert("po".to_string(), "\u{307d}".to_string()); // ぽ
    d.insert("ma".to_string(), "\u{307e}".to_string()); // ま
    d.insert("mi".to_string(), "\u{307f}".to_string()); // み
    d.insert("mu".to_string(), "\u{3080}".to_string()); // む
    d.insert("me".to_string(), "\u{3081}".to_string()); // め
    d.insert("mo".to_string(), "\u{3082}".to_string()); // も
    d.insert("ya".to_string(), "\u{3084}".to_string()); // や
    d.insert("yu".to_string(), "\u{3086}".to_string()); // ゆ
    d.insert("yo".to_string(), "\u{3088}".to_string()); // よ
    d.insert("ra".to_string(), "\u{3089}".to_string()); // ら
    d.insert("ri".to_string(), "\u{308a}".to_string()); // り
    d.insert("ru".to_string(), "\u{308b}".to_string()); // る
    d.insert("re".to_string(), "\u{308c}".to_string()); // れ
    d.insert("ro".to_string(), "\u{308d}".to_string()); // ろ
    d.insert("wa".to_string(), "\u{308f}".to_string()); // わ
    d.insert("kya".to_string(), "\u{304d}\u{3083}".to_string()); // き
    d.insert("kyu".to_string(), "\u{304d}\u{3085}".to_string()); // き
    d.insert("kyo".to_string(), "\u{304d}\u{3087}".to_string()); // き
    d.insert("gya".to_string(), "\u{304e}\u{3083}".to_string()); //
    d.insert("gyu".to_string(), "\u{304e}\u{3085}".to_string()); //
    d.insert("gyo".to_string(), "\u{304e}\u{3087}".to_string()); //
    d.insert("sha".to_string(), "\u{3057}\u{3083}".to_string()); //
    d.insert("shu".to_string(), "\u{3057}\u{3085}".to_string()); //
    d.insert("sho".to_string(), "\u{3057}\u{3087}".to_string()); //
    d.insert("ja".to_string(), "\u{3058}\u{3083}".to_string()); //
    d.insert("ju".to_string(), "\u{3058}\u{3085}".to_string()); //
    d.insert("jo".to_string(), "\u{3058}\u{3087}".to_string()); //
    d.insert("cha".to_string(), "\u{3061}\u{3083}".to_string()); //
    d.insert("chu".to_string(), "\u{3061}\u{3085}".to_string()); //
    d.insert("cho".to_string(), "\u{3061}\u{3087}".to_string()); //
    d.insert("dja".to_string(), "\u{3062}\u{3083}".to_string()); //
    d.insert("dju".to_string(), "\u{3062}\u{3085}".to_string()); //
    d.insert("djo".to_string(), "\u{3062}\u{3087}".to_string()); //
    d.insert("hya".to_string(), "\u{3072}\u{3083}".to_string()); //
    d.insert("hyu".to_string(), "\u{3072}\u{3085}".to_string()); //
    d.insert("hyo".to_string(), "\u{3072}\u{3087}".to_string()); //
    d.insert("bya".to_string(), "\u{3073}\u{3083}".to_string()); //
    d.insert("byu".to_string(), "\u{3073}\u{3085}".to_string()); //
    d.insert("byo".to_string(), "\u{3073}\u{3087}".to_string()); //
    d.insert("pya".to_string(), "\u{3074}\u{3083}".to_string()); //
    d.insert("pyu".to_string(), "\u{3074}\u{3085}".to_string()); //
    d.insert("pyo".to_string(), "\u{3074}\u{3087}".to_string()); //
    d.insert("nya".to_string(), "\u{306b}\u{3083}".to_string()); //
    d.insert("nyu".to_string(), "\u{306b}\u{3085}".to_string()); //
    d.insert("nyo".to_string(), "\u{306b}\u{3087}".to_string()); //
    d.insert("wi".to_string(), "\u{3090}".to_string()); // ゐ
    d.insert("we".to_string(), "\u{3091}".to_string()); // ゑ
    d.insert("wo".to_string(), "\u{3092}".to_string()); // を
    d.insert("n".to_string(), "\u{3093}".to_string()); // ん
    d.insert("vu".to_string(), "\u{3094}".to_string()); // ゔ

    // Katakana
    d.insert("A".to_string(), "\u{30a2}".to_string()); // ア
    d.insert("I".to_string(), "\u{30a4}".to_string()); // イ
    d.insert("U".to_string(), "\u{30a6}".to_string()); // ウ
    d.insert("E".to_string(), "\u{30a8}".to_string()); // エ
    d.insert("O".to_string(), "\u{30aa}".to_string()); // オ
    d.insert("KA".to_string(), "\u{30ab}".to_string()); // カ
    d.insert("GA".to_string(), "\u{30ac}".to_string()); // ガ
    d.insert("KI".to_string(), "\u{30ad}".to_string()); // キ
    d.insert("GI".to_string(), "\u{30ae}".to_string()); // ギ
    d.insert("KU".to_string(), "\u{30af}".to_string()); // ク
    d.insert("GU".to_string(), "\u{30b0}".to_string()); // グ
    d.insert("KE".to_string(), "\u{30b1}".to_string()); // ケ
    d.insert("GE".to_string(), "\u{30b2}".to_string()); // ゲ
    d.insert("KO".to_string(), "\u{30b3}".to_string()); // コ
    d.insert("GO".to_string(), "\u{30b4}".to_string()); // ゴ
    d.insert("SA".to_string(), "\u{30b5}".to_string()); // サ
    d.insert("ZA".to_string(), "\u{30b6}".to_string()); // ザ
    d.insert("SHI".to_string(), "\u{30b7}".to_string()); // シ
    d.insert("JI".to_string(), "\u{30b8}".to_string()); // ジ
    d.insert("SU".to_string(), "\u{30b9}".to_string()); // ス
    d.insert("ZU".to_string(), "\u{30ba}".to_string()); // ズ
    d.insert("SE".to_string(), "\u{30bb}".to_string()); // セ
    d.insert("ZE".to_string(), "\u{30bc}".to_string()); // ゼ
    d.insert("SO".to_string(), "\u{30bd}".to_string()); // ソ
    d.insert("ZO".to_string(), "\u{30be}".to_string()); // ゾ
    d.insert("TA".to_string(), "\u{30bf}".to_string()); // タ
    d.insert("DA".to_string(), "\u{30c0}".to_string()); // ダ
    d.insert("CHI".to_string(), "\u{30c1}".to_string()); // チ
    d.insert("DJI".to_string(), "\u{30c2}".to_string()); // ヂ
    d.insert("TSU".to_string(), "\u{30c4}".to_string()); // ツ
    d.insert("DZU".to_string(), "\u{30c5}".to_string()); // ヅ
    d.insert("TE".to_string(), "\u{30c6}".to_string()); // テ
    d.insert("DE".to_string(), "\u{30c7}".to_string()); // デ
    d.insert("TO".to_string(), "\u{30c8}".to_string()); // ト
    d.insert("DO".to_string(), "\u{30c9}".to_string()); // ド
    d.insert("NA".to_string(), "\u{30ca}".to_string()); // ナ
    d.insert("NI".to_string(), "\u{30cb}".to_string()); // ニ
    d.insert("NU".to_string(), "\u{30cc}".to_string()); // ヌ
    d.insert("NE".to_string(), "\u{30cd}".to_string()); // ネ
    d.insert("NO".to_string(), "\u{30ce}".to_string()); // ノ
    d.insert("HA".to_string(), "\u{30cf}".to_string()); // ハ
    d.insert("BA".to_string(), "\u{30d0}".to_string()); // バ
    d.insert("PA".to_string(), "\u{30d1}".to_string()); // パ
    d.insert("HI".to_string(), "\u{30d2}".to_string()); // ヒ
    d.insert("BI".to_string(), "\u{30d3}".to_string()); // ビ
    d.insert("PI".to_string(), "\u{30d4}".to_string()); // ピ
    d.insert("FU".to_string(), "\u{30d5}".to_string()); // フ
    d.insert("BU".to_string(), "\u{30d6}".to_string()); // ブ
    d.insert("PU".to_string(), "\u{30d7}".to_string()); // プ
    d.insert("HE".to_string(), "\u{30d8}".to_string()); // ヘ
    d.insert("BE".to_string(), "\u{30d9}".to_string()); // ベ
    d.insert("PE".to_string(), "\u{30da}".to_string()); // ペ
    d.insert("HO".to_string(), "\u{30db}".to_string()); // ホ
    d.insert("BO".to_string(), "\u{30dc}".to_string()); // ボ
    d.insert("PO".to_string(), "\u{30dd}".to_string()); // ポ
    d.insert("MA".to_string(), "\u{30de}".to_string()); // マ
    d.insert("MI".to_string(), "\u{30df}".to_string()); // ミ
    d.insert("MU".to_string(), "\u{30e0}".to_string()); // ム
    d.insert("ME".to_string(), "\u{30e1}".to_string()); // メ
    d.insert("MO".to_string(), "\u{30e2}".to_string()); // モ
    d.insert("YA".to_string(), "\u{30e4}".to_string()); // ヤ
    d.insert("YU".to_string(), "\u{30e6}".to_string()); // ユ
    d.insert("YO".to_string(), "\u{30e8}".to_string()); // ヨ
    d.insert("RA".to_string(), "\u{30e9}".to_string()); // ラ
    d.insert("RI".to_string(), "\u{30ea}".to_string()); // リ
    d.insert("RU".to_string(), "\u{30eb}".to_string()); // ル
    d.insert("RE".to_string(), "\u{30ec}".to_string()); // レ
    d.insert("RO".to_string(), "\u{30ed}".to_string()); // ロ
    d.insert("WA".to_string(), "\u{30ef}".to_string()); // ワ
    d.insert("KYA".to_string(), "\u{30ad}\u{30e3}".to_string()); //
    d.insert("KYU".to_string(), "\u{30ad}\u{30e5}".to_string()); //
    d.insert("KYO".to_string(), "\u{30ad}\u{30e7}".to_string()); //
    d.insert("GYA".to_string(), "\u{30ae}\u{30e3}".to_string()); //
    d.insert("GYU".to_string(), "\u{30ae}\u{30e5}".to_string()); //
    d.insert("GYO".to_string(), "\u{30ae}\u{30e7}".to_string()); //
    d.insert("SHA".to_string(), "\u{30b7}\u{30e3}".to_string()); //
    d.insert("SHU".to_string(), "\u{30b7}\u{30e5}".to_string()); //
    d.insert("SHO".to_string(), "\u{30b7}\u{30e7}".to_string()); //
    d.insert("JA".to_string(), "\u{30b8}\u{30e3}".to_string()); //
    d.insert("JU".to_string(), "\u{30b8}\u{30e5}".to_string()); //
    d.insert("JO".to_string(), "\u{30b8}\u{30e7}".to_string()); //
    d.insert("CHA".to_string(), "\u{30c1}\u{30e3}".to_string()); //
    d.insert("CHU".to_string(), "\u{30c1}\u{30e5}".to_string()); //
    d.insert("CHO".to_string(), "\u{30c1}\u{30e7}".to_string()); //
    d.insert("DJA".to_string(), "\u{30c2}\u{30e3}".to_string()); //
    d.insert("DJU".to_string(), "\u{30c2}\u{30e5}".to_string()); //
    d.insert("DJO".to_string(), "\u{30c2}\u{30e7}".to_string()); //
    d.insert("HYA".to_string(), "\u{30d2}\u{30e3}".to_string()); //
    d.insert("HYU".to_string(), "\u{30d2}\u{30e5}".to_string()); //
    d.insert("HYO".to_string(), "\u{30d2}\u{30e7}".to_string()); //
    d.insert("BYA".to_string(), "\u{30d3}\u{30e3}".to_string()); //
    d.insert("BYU".to_string(), "\u{30d3}\u{30e5}".to_string()); //
    d.insert("BYO".to_string(), "\u{30d3}\u{30e7}".to_string()); //
    d.insert("PYA".to_string(), "\u{30d4}\u{30e3}".to_string()); //
    d.insert("PYU".to_string(), "\u{30d4}\u{30e5}".to_string()); //
    d.insert("PYO".to_string(), "\u{30d4}\u{30e7}".to_string()); //
    d.insert("NYA".to_string(), "\u{30cb}\u{30e3}".to_string()); //
    d.insert("NYU".to_string(), "\u{30cb}\u{30e5}".to_string()); //
    d.insert("NYO".to_string(), "\u{30cb}\u{30e7}".to_string()); //
    d.insert("WI".to_string(), "\u{30f0}".to_string()); // ヰ
    d.insert("WE".to_string(), "\u{30f1}".to_string()); // ヱ
    d.insert("WO".to_string(), "\u{30f2}".to_string()); // ヲ
    d.insert("N".to_string(), "\u{30f3}".to_string()); // ン
    d.insert("VU".to_string(), "\u{30f4}".to_string()); // ヴ

    // Punctuation
    d.insert(".".to_string(), "\u{3002}".to_string()); // 。
    d.insert(",".to_string(), "\u{3001}".to_string()); // 、
    d.insert("!".to_string(), "\u{ff01}".to_string()); // ！
    d.insert("?".to_string(), "\u{ff1f}".to_string()); // ？
}

pub fn new() -> Dict {
    Dict::with_capacity(212)
}
