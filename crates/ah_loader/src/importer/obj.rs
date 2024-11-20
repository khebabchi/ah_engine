use std::fs::File;
use std::io::BufReader;
use obj::{Obj, ObjResult};

fn load_obj(filename: &str) -> Obj {
        use std::fs::File;
        use std::io::BufReader;
        use obj::{load_obj, Obj};

        let input = BufReader::new(File::open("tests/fixtures/dome.obj").expect("obj file not found"));
        load_obj(input).expect(&format!("failed to load {}", filename))
}