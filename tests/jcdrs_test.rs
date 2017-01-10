extern crate jcdrs;

fn test_init() -> dict::Dict {
    dict::init(dict::new())
}

#[test]
fn test1() {
    //let mut map = dict::new();
    //dict::init(&mut map);
    let map = test_init();
    println!("map length: {}", map.len());

    for (key, val) in map.iter() {
        println!("{}\t{}", key, val);
    }
    println!("derp");
    assert!(true);
}
