extern crate libnb;
use std::fs::File;

#[test]
fn test_parser() {
    let mut file = File::open("rdb/dump.rdb").unwrap();
    let mut dparser = libnb::DefaultRdbParser::default();
    let parsed = dparser.read_to_cmd(&mut file).unwrap();
    for cmd in parsed {
        println!("cmd: {:?}", cmd);
    }
}
