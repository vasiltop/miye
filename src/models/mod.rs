use obj::{load_obj, Obj};
use std::fs::File;
use std::io::BufReader;

pub fn load_model(file_path: &str) -> Obj {
    let input = BufReader::new(File::open(file_path).unwrap());

    load_obj(input).unwrap()
}
