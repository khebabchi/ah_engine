use std::fs::File;
use std::io::BufReader;
use obj::{load_obj, Obj};
fn loader_obj(filename: &str) -> Obj {

        let input = BufReader::new(File::open("tests/fixtures/dome.obj").expect("obj file not found"));
        load_obj(input).expect(&format!("failed to load {}", filename))
}